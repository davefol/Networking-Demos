use clap::Parser;
use pnet::datalink;
use tabled::builder::Builder;

/// List interfaces
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, action)]
    description: bool,
    #[clap(long, action)]
    index: bool,
    #[clap(long, action)]
    mac: bool,
    #[clap(long, action)]
    ips: bool,
}

pub fn list_interfaces(args: Args) {
    let mut cols = 1;
    if args.description {
        cols += 1;
    }
    if args.index {
        cols += 1;
    }
    if args.mac {
        cols += 1;
    }
    if args.ips {
        cols += 1;
    }

    let ifaces = datalink::interfaces();

    let mut b = Builder::with_capacity(ifaces.len(), cols);
    for iface in ifaces {
        let mut record = vec![iface.name];
        if args.description {
            record.push(iface.description);
        }
        if args.index {
            record.push(format!("{}", iface.index))
        }
        if args.mac {
            if let Some(mac) = iface.mac {
                record.push(format!("{}", mac));
            } else {
                record.push("None".into());
            }
        }
        if args.ips {
            record.push(
                iface
                    .ips
                    .iter()
                    .map(|ip| format!("{}", ip))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
        b.push_record(record);
    }
    println!("{}", b.build());
}
