import { promises as fs } from "fs";
import path from "path";
import chalk from "chalk";
import { ConfigManager } from "./config.js";

/**
 * Neo-Forge - Foundry-like UX for Neo-Solidity projects.
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

    console.log(chalk.blue("🧪 neo-forge test (scaffold)"));
    console.log(`  test dir: ${profile.test}`);

    if (options.pattern) console.log(`  pattern: ${options.pattern}`);
    if (options.gasReport) console.log("  gas report: enabled (placeholder)");
    if (options.coverage) console.log("  coverage: enabled (placeholder)");
    if (options.forkUrl) console.log(`  fork url: ${options.forkUrl}`);
    if (typeof options.forkBlockNumber === "number") {
      console.log(`  fork block: ${options.forkBlockNumber}`);
    }

    throw new Error(
      "neo-forge test is not implemented yet. Neo VM test execution is still on the roadmap."
    );
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
