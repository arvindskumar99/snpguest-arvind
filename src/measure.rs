use super::*;
use sev::measurement::{self, gctx, ovmf, sev_hashes, snp::{self, SnpMeasurementArgs}, vcpu_types, vmsa};
use std::{fmt, str::FromStr};
use anyhow::{anyhow, Result};


use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use std::path::{Path, PathBuf};
use clap::{Args, Parser, builder, Subcommand};


#[derive(Parser, Debug, , Subcommand)]
#[command(author, version, about, long_about = None)]
pub struct MeasureArgs{

#[arg(short, long, help = "Kernel command line to calculate hash from (use with --kernel)")]
pub append: u64,    
    

}



pub fn cmd(args: MeasureArgs) ->  Result<()>{
    let mut args = MeasureArgs::parse();
    println!("{:?}", args);

    // let vcpu_type = vcpu_types::CpuType::from_str(args.vcpu_type.as_str())?;


    
    // let collected_args:SnpMeasurementArgs = measurement::snp+:SnpMeasurementArgs {
    //     vcpus: args.vcpus,
    //     vcpu_type: args.vcpu_type,
    //     ovmf_file: args.ovmf_path,
    //     guest_features: vmsa::GuestFeatures(args.guest_features),
    //     kernel_file: args.kernel,
    //     initrd_file: args.initrd,
    //     append: Some(&*args.append),
    //     ovmf_hash_str: Some(args.snp_ovmf_hash.as_str()),
    //     vmm_type: Some(measurement::vmsa::VMMType::try_from(args.vmm_type)?)
    // };


    // match sev::measurement::snp::snp_calc_launch_digest(collected_args) {
    //     Ok(ld) => println!("{:?}", ld),
    //     Err(_) => return Err(anyhow!("Error calculating the measurement."))
    // };

    Ok(())

    

    
}







// #[clap(long, help = "Show program's version number and exit")]
// pub version: String,

// #[arg(long, help = "{snp,snp:ovmf-hash,snp:svsm}", value_parser = builder::PossibleValuesParser::new(["snp", "snp:ovmf-hash", "snp:svsm"]) )]
// pub mode: String,

// #[arg(long, help = "Number of guest vcpus")]
// pub vcpus: u32,

// #[arg(long, help = "Type of guest vcpu (EPYC, EPYC-v1, EPYC-v2, EPYC-IBPB, EPYC-v3, EPYC-v4,
//     EPYC-Rome, EPYC-Rome-v1, EPYC-Rome-v2, EPYC-Rome-v3, EPYC-Milan, EPYC-
//     Milan-v1, EPYC-Milan-v2, EPYC-Genoa, EPYC-Genoa-v1)")]
// pub vcpu_type: String,

// // #[arg(long, help = "Guest vcpu signature value")]
// // pub vcpu_sig: String,

// // #[arg(long, help = "Guest vcpu family")]
// // pub vcpu_family: String,

// // #[arg(long, help = "Guest vcpu model")]
// // pub vcpu_model: String,

// #[arg(long, help = "Type of guest vmm (1 = QEMU, 2 = ec2, 3 = KRUN)", value_parser = builder::PossibleValuesParser::new(["1", "2", "3"]))]
// pub vmm_type: u8,

// #[arg(long, help = "OVMF file to calculate hash from")]
// pub ovmf_path: PathBuf,

// #[arg(long, help = "Hex representation of the guest kernel features expected to be included
// (defaults to 0x1); see README.md for possible values", default_value = "0x1")]
// pub guest_features: u64,

// // #[arg(long, help = "Measurement output format {hex, base64}")]
// // pub output_format: String,

// #[arg(long, help = "Precalculated hash of the OVMF binary (hex string)", value_parser)]
// pub snp_ovmf_hash: String,

// // #[arg(long, help = "Write measured VMSAs to +vmsa<N>.bin (seves, snp, and snp:svsm modes only)")]
// // pub dump_vmsa: String,

// // #[arg(long, help = "SVSM binary", required_if = ("mode", "snp:svsm"))]
// // pub svsm_path: Option<PathBuf>,

// // #[arg(long, help = "Size of the OVMF_VARS file in bytes (conflicts with --vars-file)", 
// //     required_if = ("mode", "snp:svsm"))]
// // pub vars_size: u64,

// // #[arg(long, help = "OVMF_VARS file (conflicts with --vars-size)", required_if = ("mode", "snp:svsm"))]
// // pub vars_file: Option<PathBuf>
// #[arg(long, help = "Initrd file to calculate hash from (use with --kernel)")]
// pub initrd: Option<PathBuf>,


// #[arg(long, help = "Kernel file to calculate hash from", requires = ("append"), requires = ("initrd"))]
// pub kernel: Option<PathBuf>, 

