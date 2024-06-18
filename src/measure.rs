use super::*;
use anyhow::{anyhow, Result};
use clap::Parser;
use clap_num::maybe_hex;
use sev::measurement::{self, snp::SnpMeasurementArgs, vmsa};
use std::path::PathBuf;
use hex;
use base64::{engine::general_purpose, Engine as _};




#[derive(Parser, Debug)]
pub struct MeasureArgs {
    ///Kernel command line to calculate hash from (use with --kernel)
    #[arg(short, long, value_name = "append")]
    pub append: Option<String>,

    /// {snp,snp:ovmf-hash,snp:svsm}
    #[arg(short, long, value_name = "mode")]
    pub mode: Option<String>,

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
    pub ovmf_path: Option<PathBuf>,

    /// Hex representation of the guest kernel features expected to be included
    #[arg(short, long, value_name = "guest-features", default_value_t = 1u64, value_parser=maybe_hex::<u64>)]
    pub guest_features: u64,

    /// Precalculated hash of the OVMF binary (hex string)
    #[arg(short, long, value_name = "snp-ovmf-hash")]
    pub snp_ovmf_hash: Option<String>,

    /// Initrd file to calculate hash from (use with --kernel)
    #[arg(short, long, value_name = "initrd")]
    pub initrd: Option<PathBuf>,
    
    ///Guest vcpu signature value
    #[arg(long, value_name = "vcpu-sig")]
    pub vcpu_sig: Option<i32>,

    /// Guest vcpu family
    #[arg(long, value_name = "vcpu-family")]
    pub vcpu_family: Option<i32>,
    
    /// Guest vcpu model
    #[arg(long, value_name = "vcpu-model")]
    pub vcpu_model: Option<i32>,

    /// Guest vcpu stepping
    #[arg(long, value_name = "vcpu-stepping")]
    pub vcpu_stepping: Option<i32>,

    ///Choose output format (base64, hex)
    #[arg(long, value_name = "output-format", default_value = "hex")]
    pub output_format: String,

    /// Kernel file to calculate hash from
    #[arg(short, long, requires = "append", requires = "initrd", value_name = "kernel")]
    pub kernel: Option<PathBuf>,
}


//pass sig, model, stepping, family, stepping model and family need ot be passed together, and then it's just vcputype or just signature.
pub fn get_vcpu_sig(args: &MeasureArgs) -> Result<i32, anyhow::Error> {
    if args.vcpu_family.is_some() {
        return Ok(measurement::vcpu_types::cpu_sig(args.vcpu_family.expect("family"), 
                                                args.vcpu_model.expect("model"),
                                                args.vcpu_stepping.expect("stepping")))
    } else if args.vcpu_sig.is_some() {
        return Ok(args.vcpu_sig.unwrap())
    } else if !args.vcpu_type.is_empty() {
        return Ok(measurement::vcpu_types::CpuType::try_from(args.vcpu_type.as_str())?.sig())
    } else{
        return Err(anyhow::anyhow!("Missing either VCPU type, signature, or family"))
    }
}


pub fn get_measurement(args: MeasureArgs) -> Result<()> {
    let vcpu_sig = get_vcpu_sig(&args);
    println!("{:?}", args);
    let append = args.append.unwrap();
    
    let ovmf;
    let test;

    if args.snp_ovmf_hash.is_none(){
        test = None;
    }
    else {
        ovmf = args.snp_ovmf_hash.unwrap_or(String::from(""));
        test = Some(ovmf.as_str());
    }

    let collected_args: SnpMeasurementArgs = measurement::snp::SnpMeasurementArgs {
        vcpus: args.vcpus,
        vcpu_type: measurement::vcpu_types::CpuType::try_from(args.vcpu_type.as_str())?,
        ovmf_file: args.ovmf_path.unwrap(),
        guest_features: vmsa::GuestFeatures(args.guest_features),
        kernel_file: args.kernel,
        initrd_file: args.initrd,
        append: Some(append.as_str()),
        ovmf_hash_str: test,
        vmm_type: Some(measurement::vmsa::VMMType::try_from(args.vmm_type)?),
    };

    match sev::measurement::snp::snp_calc_launch_digest(collected_args) {
        Ok(ld) => {
        if args.output_format == "hex"{
            println!("{:x?}", hex::encode(ld))
        }else {
            println!("{:?}", general_purpose::STANDARD.encode(ld))
        }
        },
        Err(e) => return Err(anyhow!("Error calculating the measurement:{e}")),
    };

    Ok(())
}
