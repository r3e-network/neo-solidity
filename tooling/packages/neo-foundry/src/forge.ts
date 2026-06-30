import { promises as fs } from "fs";
import path from "path";
import { spawn } from "child_process";
import chalk from "chalk";
import { ConfigManager } from "./config.js";

/**
 * Resolve the `neo-test` runner binary: the `NEO_TEST` env var wins, otherwise
 * `neo-test` is expected on `PATH`. (Build it from this repo with
 * `cargo build --release --bin neo-test`.)
 */
function resolveNeoTestBin(): string {
  return process.env.NEO_TEST || "neo-test";
}

function runNeoTest(bin: string, args: string[]): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = spawn(bin, args, { stdio: "inherit" });
    child.on("error", (err: NodeJS.ErrnoException) => {
      if (err.code === "ENOENT") {
        reject(
          new Error(
            `neo-test binary not found ('${bin}'). Build it with ` +
              "`cargo build --release --bin neo-test` and put it on PATH, " +
              "or set the NEO_TEST environment variable to its path."
          )
        );
      } else {
        reject(err);
      }
    });
    child.on("close", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`neo-test exited with code ${code ?? "unknown"}`));
    });
  });
}

/**
 * Neo-Forge - Foundry-like UX for Neo DevPack for Solidity projects.
 *
 * This package is intentionally a scaffold: the CLI shows the intended workflow
 * and validates configuration, but does not yet wire into a compiler + test VM.
 */
export class NeoForge {
  private readonly config: ConfigManager;
  private readonly profileName: string;

  constructor(configPath?: string, profileName = "default") {
    this.config = new ConfigManager(configPath);
    this.profileName = profileName;
  }

  async build(options: {
    force?: boolean;
    watch?: boolean;
    profile?: string;
    quiet?: boolean;
  } = {}): Promise<void> {
    const config = await this.config.loadConfig();
    const profile = this.config.getProfile(options.profile || this.profileName);

    void config;

    if (!options.quiet) {
      console.log(chalk.blue("🔧 neo-forge build (scaffold)"));
      console.log(`  src: ${profile.src}`);
      console.log(`  out: ${profile.out}`);
      if (options.watch) {
        console.log(chalk.yellow("  watch mode is not implemented yet"));
      }
    }

    throw new Error(
      "neo-forge build is not implemented yet. Use `neo-solc` directly (or the Hardhat plugin) to compile."
    );
  }

  async test(options: {
    pattern?: string;
    verbose?: boolean;
    gasReport?: boolean;
    coverage?: boolean;
    forkUrl?: string;
    forkBlockNumber?: number;
    profile?: string;
  } = {}): Promise<void> {
    await this.config.loadConfig();
    const profile = this.config.getProfile(options.profile || this.profileName);
    const testDir = profile.test || "test";

    // Delegate to the native `neo-test` runner: it compiles each *.t.sol with
    // neo-solc and executes every test*()/setUp() on the in-tree NeoVM.
    const args: string[] = [testDir];
    if (options.pattern) args.push("--match-test", options.pattern);
    if (options.gasReport) args.push("--gas");
    if (options.verbose) args.push("-v");
    if (options.coverage) {
      console.log(chalk.yellow("  (coverage is not yet supported by neo-test — ignoring --coverage)"));
    }
    if (options.forkUrl || typeof options.forkBlockNumber === "number") {
      console.log(chalk.yellow("  (forking is not yet supported by neo-test — ignoring --fork-*)"));
    }

    await runNeoTest(resolveNeoTestBin(), args);
  }

  async clean(profileName?: string): Promise<void> {
    await this.config.loadConfig();
    const profile = this.config.getProfile(profileName || this.profileName);

    console.log(chalk.blue("🧹 Cleaning build artifacts..."));

    await this.rmrf(profile.out);
    await this.rmrf(profile.build.cacheDir);

    console.log(chalk.green("✅ Cleaning completed"));
  }

  async init(projectPath = "."): Promise<void> {
    console.log(chalk.blue("🚀 Initializing Neo-Foundry project..."));
    await this.config.initProject(projectPath);
    console.log(chalk.green("✅ Project initialized successfully!"));
  }

  async install(dependencies: string[]): Promise<void> {
    console.log(chalk.blue(`📦 neo-forge install (scaffold) (${dependencies.length} deps)`));
    void dependencies;
    throw new Error("neo-forge install is not implemented yet.");
  }

  async remove(dependency: string): Promise<void> {
    console.log(chalk.blue(`🗑️  neo-forge remove (scaffold): ${dependency}`));
    throw new Error("neo-forge remove is not implemented yet.");
  }

  async update(): Promise<void> {
    console.log(chalk.blue("🔄 neo-forge update (scaffold)"));
    throw new Error("neo-forge update is not implemented yet.");
  }

  async inspect(contract: string, options: { pretty?: boolean } = {}): Promise<void> {
    await this.config.loadConfig();
    void options;

    console.log(chalk.blue(`🔎 neo-forge inspect (scaffold): ${contract}`));
    throw new Error("neo-forge inspect is not implemented yet.");
  }

  private async rmrf(dir: string): Promise<void> {
    const full = path.isAbsolute(dir) ? dir : path.join(process.cwd(), dir);
    await fs.rm(full, { recursive: true, force: true });
  }
}
