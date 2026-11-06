use std::{collections::HashMap,fs,net::{IpAddr,Ipv4Addr,Ipv6Addr}};
#[derive(Debug)]struct Socket{protocol:String,address:IpAddr,port:u16,uid:u32,inode:u64}
fn endpoint(text:&str)->Result<(IpAddr,u16),String>{
 let(ip,port)=text.split_once(':').ok_or("Missing port")?;let port=u16::from_str_radix(port,16).map_err(|_|"Invalid port")?;
 let word=|s:&str|u32::from_str_radix(s,16).map_err(|_|"Invalid address".to_string());
 let address=match ip.len(){8=>IpAddr::V4(Ipv4Addr::from(word(ip)?.to_ne_bytes())),32=>{let mut bytes=[0u8;16];for i in 0..4{bytes[i*4..i*4+4].copy_from_slice(&word(&ip[i*8..i*8+8])?.to_ne_bytes());}IpAddr::V6(Ipv6Addr::from(bytes))},_=>return Err("Invalid address length".into())};Ok((address,port))
}
fn parse(protocol:&str,text:&str)->Vec<Socket>{
 text.lines().skip(1).filter_map(|line|{let v:Vec<&str>=line.split_whitespace().collect();if v.len()<10{return None}
 if(protocol.starts_with("tcp")&&v[3]!="0A")||(protocol.starts_with("udp")&&v[3]!="07"){return None}
 let(address,port)=endpoint(v[1]).ok()?;Some(Socket{protocol:protocol.into(),address,port,uid:v[7].parse().ok()?,inode:v[9].parse().ok()?})}).collect()
}
fn owners()->HashMap<u64,Vec<String>>{
 let mut out:HashMap<u64,Vec<String>>=HashMap::new();if let Ok(entries)=fs::read_dir("/proc"){for e in entries.flatten(){
 let pid=e.file_name().to_string_lossy().to_string();if pid.parse::<u32>().is_err(){continue}
 let name=fs::read_to_string(e.path().join("comm")).unwrap_or_else(|_|"?".into()).trim().chars().map(|c|if c.is_control(){'?'}else{c}).collect::<String>();
 if let Ok(fds)=fs::read_dir(e.path().join("fd")){for fd in fds.flatten(){if let Ok(target)=fs::read_link(fd.path()){
 let value=target.to_string_lossy();if let Some(inode)=value.strip_prefix("socket:[").and_then(|s|s.strip_suffix(']')).and_then(|s|s.parse::<u64>().ok()){
 let entry=out.entry(inode).or_default();let label=format!("{pid}/{name}");if !entry.contains(&label){entry.push(label);}
 }}}}
 }}out
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 if !cfg!(target_os="linux"){return Err("Linux procfs is required".into())}
 let args:Vec<String>=std::env::args().skip(1).collect();let mut filter=None;let mut i=0;
 while i<args.len(){match args[i].as_str(){"--port"=>{i+=1;filter=Some(args.get(i).ok_or("--port needs a number")?.parse::<u16>()?);},"--help"=>{println!("port-atlas [--port NUMBER]\nTCP listeners and unconnected bound UDP sockets in the current network namespace.");return Ok(())},_=>return Err("Unknown option".into())}i+=1;}
 let mut sockets=Vec::new();let mut readable=0;
 for protocol in ["tcp","tcp6","udp","udp6"]{if let Ok(text)=fs::read_to_string(format!("/proc/net/{protocol}")){readable+=1;sockets.extend(parse(protocol,&text));}}
 if readable==0{return Err("No readable /proc/net socket tables".into())}
 let owners=owners();sockets.sort_by_key(|s|(s.port,s.protocol.clone()));println!("PROTO  ENDPOINT                                     UID  OWNERS (PID/name)");
 for s in sockets.iter().filter(|s|filter.is_none()||filter==Some(s.port)){
 let endpoint=match s.address{IpAddr::V6(_)=>format!("[{}]:{}",s.address,s.port),_=>format!("{}:{}",s.address,s.port)};
 println!("{:<6} {:<42} {:>6}  {}",s.protocol,endpoint,s.uid,owners.get(&s.inode).map(|v|v.join(", ")).unwrap_or_else(||"unknown/inaccessible".into()));
 }println!("Read-only snapshot; owner visibility depends on procfs permissions.");Ok(())
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn decodes_addresses(){assert_eq!(endpoint("0100007F:1F90").unwrap(),("127.0.0.1".parse().unwrap(),8080));assert_eq!(endpoint("00000000000000000000000001000000:0016").unwrap(),("::1".parse().unwrap(),22));assert!(endpoint("oops:1").is_err());}
 #[test]fn keeps_listeners_and_udp_bound_only(){let text="head\n 0: 0100007F:0016 00000000:0000 0A 0:0 0:0 0 1000 0 42\n 1: 0100007F:1234 00000000:0000 01 0:0 0:0 0 1000 0 43\n";let rows=parse("tcp",text);assert_eq!(rows.len(),1);assert_eq!((rows[0].port,rows[0].inode),(22,42));assert_eq!(parse("udp",&text.replace("0A","07")).len(),1);}
}
