use super::*;
use anyhow::{anyhow, Result};
use sev::measurement::{self, snp::SnpMeasurementArgs, vcpu_types, vmsa};
use std::str::FromStr;

use clap::{builder, Parser};
use clap_num::maybe_hex;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct MeasureArgs {
    ///Kernel command line to calculate hash from (use with --kernel)
    #[arg(short, long, value_name = "append")]
    pub append: String,

    /// {snp,snp:ovmf-hash,snp:svsm}
    #[arg(short, long, value_name = "mode")]
    pub mode: String,

    /// Number of guest vcpus
    #[arg(long)]
    pub vcpus: u32,

    ///Type of guest vcpu (EPYC, EPYC-v1, EPYC-v2, EPYC-IBPB, EPYC-v3, EPYC-v4,
    ///    EPYC-Rome, EPYC-Rome-v1, EPYC-Rome-v2, EPYC-Rome-v3, EPYC-Milan, EPYC-
    ///    Milan-v1, EPYC-Milan-v2, EPYC-Genoa, EPYC-Genoa-v1)
    #[arg(short, long, value_name = "vcpu-type")]
    pub vcpu_type: String,

    /// Type of guest vmm (1 = QEMU, 2 = ec2, 3 = KRUN)
    #[arg(long, value_name = "vmm-type")]
    pub vmm_type: u8,

    /// OVMF file to calculate hash from
    #[arg(short, long, value_name = "ovmf-path")]
    pub ovmf_path: PathBuf,

    /// Precalculated hash of the OVMF binary (hex string)
    #[arg(short, long, value_name = "snp-ovmf-hash")]
    pub snp_ovmf_hash: String,

    /// Initrd file to calculate hash from (use with --kernel)
    #[arg(short, long, value_name = "initrd")]
    pub initrd: Option<PathBuf>,

    /// Kernel file to calculate hash from
    #[arg(short, long, requires = "append", requires = "initrd", value_name = "kernel")]
    pub kernel: Option<PathBuf>,

    /// Hex representation of the guest kernel features expected to be included
    #[arg(short, long, value_name = "guest-features", default_value_t = 1u64, value_parser=maybe_hex::<u64>)]
    pub guest_features: u64,
}

pub fn get_measurement(args: MeasureArgs) -> Result<()> {
    // println!("{:?}", args);

    let vcpu_type = vcpu_types::CpuType::from_str(args.vcpu_type.as_str())?;

    let collected_args: SnpMeasurementArgs = measurement::snp::SnpMeasurementArgs {
        vcpus: args.vcpus,
        vcpu_type: vcpu_type.to_string(),
        ovmf_file: args.ovmf_path,
        guest_features: vmsa::GuestFeatures(args.guest_features),
        kernel_file: args.kernel,
        initrd_file: args.initrd,
        append: Some(&*args.append),
        ovmf_hash_str: Some(args.snp_ovmf_hash.as_str()),
        vmm_type: Some(measurement::vmsa::VMMType::try_from(args.vmm_type)?),
    };

    match sev::measurement::snp::snp_calc_launch_digest(collected_args) {
        Ok(ld) => println!("{:?}", ld),
        Err(_) => return Err(anyhow!("Error calculating the measurement.")),
    };

    Ok(())
}
