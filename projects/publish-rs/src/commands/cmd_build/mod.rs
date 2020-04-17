use crate::{GithubError, commands::LegionArguments};
use clap::Parser;
#[cfg(feature = "wasm-opt")]
use wasm_opt::{OptimizationOptions, OptimizeLevel, ShrinkLevel};

#[derive(Debug, Parser)]
pub struct CommandBuild {
    /// Package to build (see `cargo help pkgid`)
    #[arg(short, long, visible_alias = "include", value_name = "packages...")]
    package: Vec<String>,
    /// Exclude packages from the build
    #[arg(long, value_name = "packages...")]
    exclude: Vec<String>,
    /// Build only this package's library
    #[arg(short, long, visible_alias = "lib")]
    library: bool,
    /// Build only the specified binary
    #[arg(short, long, visible_alias = "bin", value_name = "binaries...")]
    binary: Vec<String>,
    #[arg(long = "O0")]
    optimize_0: bool,
    #[arg(long = "O1")]
    optimize_1: bool,
    #[arg(long = "O2")]
    optimize_2: bool,
    #[arg(long = "O3")]
    optimize_3: bool,
    #[arg(long = "O4")]
    optimize_4: bool,
    /// Sent to the O5 Council, optimized by the O5 herself.
    #[arg(long = "O5")]
    optimize_5: bool,
    /// Optimize for size
    #[arg(long = "Os")]
    optimize_s: bool,
    /// Optimize for size in aggressively way
    #[arg(long = "Oz")]
    optimize_z: bool,
}

impl CommandBuild {
    #[cfg(feature = "wasm-opt")]
    pub async fn run(self, _: &LegionArguments) -> Result<(), GithubError> {
        let mut options = OptimizationOptions {
            reader: Default::default(),
            writer: Default::default(),
            inlining: Default::default(),
            passopts: Default::default(),
            passes: Default::default(),
            features: Default::default(),
            converge: false,
        };
        self.optimize_level(&mut options)?;
        println!("{:#?}", options);
        Ok(())
    }
    #[cfg(not(feature = "wasm-opt"))]
    pub async fn run(self, _: &LegionArguments) -> Result<(), GithubError> {
        println!("Unable to build because: https://github.com/dtolnay/cxx/issues/1429");
        Ok(())
    }
    #[cfg(feature = "wasm-opt")]
    pub fn optimize_level(&self, options: &mut OptimizationOptions) -> Result<(), GithubError> {
        if self.optimize_5 {
            panic!("The O5 Council does not exist!");
        }
        else if self.optimize_z {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_s {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_4 {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_3 {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_2 {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_1 {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        else if self.optimize_0 {
            options.passopts.optimize_level = OptimizeLevel::Level2;
            options.passopts.shrink_level = ShrinkLevel::Level2;
            options.passes.add_default_passes = true;
        }
        Ok(())
    }
}
