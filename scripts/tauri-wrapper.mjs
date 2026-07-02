import { spawn, spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
const tauriBinary =
  process.platform === "win32"
    ? path.resolve("node_modules", ".bin", "tauri.cmd")
    : path.resolve("node_modules", ".bin", "tauri");

const runTauri = () =>
  new Promise((resolve) => {
    const child = spawn(tauriBinary, args, {
      env: process.env,
      stdio: ["inherit", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      const text = chunk.toString();
      stdout += text;
      process.stdout.write(chunk);
    });

    child.stderr.on("data", (chunk) => {
      const text = chunk.toString();
      stderr += text;
      process.stderr.write(chunk);
    });

    child.on("close", (code) => {
      resolve({ code: code ?? 1, output: `${stdout}\n${stderr}` });
    });
  });

const isDmgBuild = () => {
  if (process.platform !== "darwin" || args[0] !== "build") {
    return false;
  }

  for (let i = 0; i < args.length; i += 1) {
    const arg = args[i];
    const next = args[i + 1];

    if (arg === "--bundles" && next?.split(",").includes("dmg")) {
      return true;
    }

    if (arg.startsWith("--bundles=") && arg.slice("--bundles=".length).split(",").includes("dmg")) {
      return true;
    }
  }

  return false;
};

const getArgValue = (name) => {
  for (let i = 0; i < args.length; i += 1) {
    if (args[i] === name) {
      return args[i + 1];
    }

    if (args[i].startsWith(`${name}=`)) {
      return args[i].slice(name.length + 1);
    }
  }

  return null;
};

const mapArch = (value) => {
  if (value === "x64") {
    return "x86_64";
  }

  if (value === "arm64") {
    return "aarch64";
  }

  return value;
};

const findNewestAppBundle = (bundleMacosDir) => {
  const candidates = readdirSync(bundleMacosDir)
    .filter((entry) => entry.endsWith(".app"))
    .map((entry) => {
      const fullPath = path.join(bundleMacosDir, entry);
      return { fullPath, entry, mtimeMs: statSync(fullPath).mtimeMs };
    })
    .sort((left, right) => right.mtimeMs - left.mtimeMs);

  if (candidates.length === 0) {
    throw new Error(`No app bundle found in ${bundleMacosDir}`);
  }

  return candidates[0];
};

const loadTauriConfig = () => {
  const configPath = path.resolve("src-tauri", "tauri.conf.json");
  return JSON.parse(readFileSync(configPath, "utf8"));
};

const fallbackDmgBuild = () => {
  const config = loadTauriConfig();
  const dmgConfig = config.bundle?.macOS?.dmg ?? {};
  const target = getArgValue("--target");
  const targetDir = target ? path.resolve("target", target, "release", "bundle") : path.resolve("target", "release", "bundle");
  const bundleMacosDir = path.join(targetDir, "macos");
  const bundleDmgDir = path.join(targetDir, "dmg");
  const scriptPath = path.join(bundleDmgDir, "bundle_dmg.sh");
  const volumeIconPath = path.join(bundleDmgDir, "icon.icns");

  if (!existsSync(scriptPath)) {
    throw new Error(`Generated DMG helper is missing: ${scriptPath}`);
  }

  const appBundle = findNewestAppBundle(bundleMacosDir);
  const arch = mapArch(target ? target.split("-")[0] : process.arch);
  const productName = config.productName;
  const version = config.version;
  const outputPath = path.join(bundleDmgDir, `${productName}_${version}_${arch}.dmg`);
  const windowSize = dmgConfig.windowSize ?? { width: 660, height: 400 };
  const appPosition = dmgConfig.appPosition ?? { x: 180, y: 170 };
  const applicationsPosition = dmgConfig.applicationFolderPosition ?? { x: 480, y: 170 };
  const scriptArgs = [
    scriptPath,
    "--volname",
    productName,
    "--icon",
    appBundle.entry,
    String(appPosition.x),
    String(appPosition.y),
    "--app-drop-link",
    String(applicationsPosition.x),
    String(applicationsPosition.y),
    "--window-size",
    String(windowSize.width),
    String(windowSize.height),
    "--hide-extension",
    appBundle.entry,
    "--volicon",
    path.relative(bundleMacosDir, volumeIconPath),
    "--skip-jenkins",
  ];

  if (dmgConfig.windowPosition) {
    scriptArgs.push(
      "--window-pos",
      String(dmgConfig.windowPosition.x),
      String(dmgConfig.windowPosition.y),
    );
  }

  if (dmgConfig.background) {
    scriptArgs.push("--background", path.resolve("src-tauri", dmgConfig.background));
  }

  scriptArgs.push(outputPath, appBundle.entry);

  rmSync(outputPath, { force: true });

  console.error("DMG fallback: rerunning bundle_dmg.sh with --skip-jenkins to avoid Finder Apple Events.");

  const result = spawnSync("bash", scriptArgs, {
    cwd: bundleMacosDir,
    env: process.env,
    stdio: "inherit",
  });

  return result.status ?? 1;
};

const main = async () => {
  const { code, output } = await runTauri();

  if (code === 0 || !isDmgBuild() || !output.includes("error running bundle_dmg.sh")) {
    process.exit(code);
  }

  try {
    process.exit(fallbackDmgBuild());
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(code);
  }
};

await main();
