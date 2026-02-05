use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::get;
use axum_client_ip::ClientIp;
use base64::prelude::BASE64_STANDARD;
use hickory_resolver::lookup::TxtLookup;
use hickory_resolver::proto::rr::RData;
use hickory_resolver::Resolver;
use std::net::IpAddr;
use std::net::{IpAddr::{V4,V6}, SocketAddr};
use base64::Engine;
use hickory_resolver::IntoName;
use axum_client_ip::ClientIpSource;

#[derive(serde::Deserialize)]
struct Config {
    ip_source: ClientIpSource,
}
struct UrlMappings {
    name: String,
    urls: Vec<String>,
    mapping: Mapping,
}
enum Mapping {
    Redirect(String),//url //ToDo: redirect base field for standart without query
    Fn(fn() -> axum::routing::MethodRouter)
}

async fn redirect_query(Path(query): Path<String>, redirect_url: &str) -> Redirect {
    Redirect::to(&redirect_url.replace("*query*", &query))
}

//async fn trains(query: String)->String {
//    let UIC_COUNTRY_CODES = [("FI", 10), ("RU", 20),("BY", 21),("UA", 22),("MD", 23),("LT", 24),("LV", 25),("EE", 26),("KZ", 27),("GE", 28),("UZ", 29),("KP", 30),("MN", 31),("VN", 32),("CN", 33),("LA", 34),("CU", 40),("AL", 41),("JP", 42),("BA", 44),("BA", 49),("BA", 50),("PL", 51),("BG", 52),("RO", 53),("CZ", 54),("HU", 55),("SK", 56),("AZ", 57),("AM", 58),("KG", 59),("IE", 60),("KR", 61),("ME", 62),("MK", 65),("TJ", 66),("TM", 67),("AF", 68),("GB", 70),("ES", 71),("RS", 72),("GR", 73),("SE", 74),("TR", 75),("NO", 76),("HR", 78),("SI", 79),("DE", 80),("AT", 81),("LU", 82),("IT", 83),("NL", 84),("CH", 85),("DK", 86),("FR", 87),("BE", 88),("TZ", 89),("EG", 90),("TN", 91),("DZ", 92),("MA", 93),("PT", 94),("IL", 95),("IR", 96),("SY", 97),("LB", 98),("IQ", 99)];
//
//}
fn data_from_txt(e:TxtLookup)->String{
    e.iter().map(|v|v.txt_data().iter().map(|bytes| String::from_utf8_lossy(bytes)).collect::<String>()).collect::<Vec<_>>().join("\n")
}
async fn ip_data(addr: IpAddr)->String{
    let resolver = Resolver::builder_tokio().expect("cant build resolver").build();
    let lookup_txt=resolver.txt_lookup(addr.into_name().unwrap().to_string().replace(".ip6.arpa", ".origin6.asn.cymru.com").replace(".in-addr.arpa", ".origin.asn.cymru.com")).await;
    let mut result=vec![format!("{} | {} | {}",match addr{V4(_)=>{"IPv4"},V6(_)=>{"IPv6"}},addr.to_string(),resolver.reverse_lookup(addr).await.map_or("cant find PTR".into(), |x|x.as_lookup().iter().filter_map(|record| {if let RData::PTR(ptr)=record{Some(ptr.0.to_string())}else{None}}).collect::<Vec<_>>().join(" + ")))];
    let txt_records=lookup_txt.map_or("Cant find TXT record".into(),|e|data_from_txt(e));
    result.push(txt_records.clone());
    let y=txt_records.replace(" ", "");
    let asns = y.split('|').into_iter().filter(|e|e.parse::<usize>().is_ok());
    for i in asns{
    if let Ok(record)=resolver.txt_lookup(format!("AS{}.asn.cymru.com",i)).await{
        result.push(data_from_txt(record));
    }
};
result.join("\n")
}
async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let config: Config = envy::from_env().unwrap();

    println!("Listening on http://{}", addr);

    let mut app = Router::new();

    let url_mappings =[
     // ======================
    // Wildcard redirects
    // ======================
    UrlMappings { name: "google images".into(), urls: vec!["/gi/{*wildcard}".into(),"/i/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.google.com/search?q=*query*&tbm=isch".into()) },

    UrlMappings { name: "google videos".into(), urls:vec!["/v/{*wildcard}".into(),"/gv/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.google.com/search?q=*query*&tbm=vid".into()) },

    UrlMappings { name: "google".into(), urls: vec!["/g/{*wildcard}".into(),"/google/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.google.com/search?q=*query*".into()) },
UrlMappings { name: "nix options".into(), urls: vec!["/nixo/{*wildcard}".into()], mapping: Mapping::Redirect("https://search.nixos.org/options?query=*query*".into()) },
UrlMappings { name: "nix pkgs".into(), urls: vec!["/nixp/{*wildcard}".into()], mapping: Mapping::Redirect("https://search.nixos.org/packages?query=*query*".into()) },
UrlMappings { name: "nix home manager".into(), urls: vec!["/nixh/{*wildcard}".into()], mapping: Mapping::Redirect("https://home-manager-options.extranix.com/?query=*query*".into()) },

UrlMappings { name: "inwx".into(), urls: vec!["/inwx/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.inwx.de/de/domain/check#search=*query*#region=DEFAULT#rc=rc1".into()) },
UrlMappings { name: "ovh".into(), urls: vec!["/ovh/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.ovh.de/cgi-bin/newOrder/order.cgi?domain_domainChooser_domain=*query*".into()) },
UrlMappings { name: "tineye".into(), urls: vec!["/tineye/{*wildcard}".into()], mapping: Mapping::Redirect("https://tineye.com/search?url=*query*".into()) },

UrlMappings { name: "madison".into(), urls: vec!["/madison/{*wildcard}".into()], mapping: Mapping::Redirect("https://qa.debian.org/madison.php?table=all&g=on&package=*query*".into()) },
UrlMappings { name: "deb".into(), urls: vec!["/deb/{*wildcard}".into()], mapping: Mapping::Redirect("https://qa.debian.org/madison.php?table=debian&g=on&package=*query*".into()) },
UrlMappings { name: "ubu".into(), urls: vec!["/ubu/{*wildcard}".into()], mapping: Mapping::Redirect("https://qa.debian.org/madison.php?table=ubuntu&g=on&package=*query*".into()) },

UrlMappings { name: "dpkg".into(), urls: vec!["/dpkg/{*wildcard}".into()], mapping: Mapping::Redirect("https://packages.debian.org/search?keywords=*query*".into()) },
UrlMappings { name: "upkg".into(), urls: vec!["/upkg/{*wildcard}".into()], mapping: Mapping::Redirect("http://packages.ubuntu.com/search?keywords=*query*".into()) },
UrlMappings { name: "apkg".into(), urls: vec!["/apkg/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.archlinux.org/packages/?q=*query*".into()) },
UrlMappings { name: "aur".into(), urls: vec!["/aur/{*wildcard}".into()], mapping: Mapping::Redirect("https://aur.archlinux.org/packages/?K=*query*".into()) },
UrlMappings { name: "repo-search".into(), urls: vec!["/repo/{*wildcard}".into()], mapping: Mapping::Redirect("https://repology.org/projects/?search=*query*".into()) },
UrlMappings { name: "repo".into(), urls: vec!["/repo".into()], mapping: Mapping::Redirect("https://repology.org/repositories/statistics".into()) },

UrlMappings { name: "fport".into(), urls: vec!["/fport/{*wildcard}".into(),"/fports/{*wildcard}".into(),"/freshports/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.freshports.org/search.php?num=20&query=*query*".into()) },

UrlMappings { name: "gpkg".into(), urls: vec!["/gpkg/{*wildcard}".into(),"/eix/{*wildcard}".into()], mapping: Mapping::Redirect("https://packages.gentoo.org/packages/search?q=*query*".into()) },

UrlMappings { name: "ssll".into(), urls: vec!["/ssll/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.ssllabs.com/ssltest/analyze.html?d=*query*&hideResults=on&latest".into()) },
UrlMappings { name: "bgp".into(), urls: vec!["/bgp/{*wildcard}".into()], mapping: Mapping::Redirect("http://bgp.he.net/search?commit=Search&search[search]=*query*".into()) },
UrlMappings { name: "tld".into(), urls: vec!["/tld/{*wildcard}".into()], mapping: Mapping::Redirect("https://tld-list.com/tld/*query*".into()) },
UrlMappings { name: "woa".into(), urls: vec!["/woa/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.wolframalpha.com/input/?i=*query*".into()) },

UrlMappings { name: "dcc".into(), urls: vec!["/dcc/{*wildcard}".into()], mapping: Mapping::Redirect("https://www.dict.cc/?s=*query*".into()) },
UrlMappings { name: "gif".into(), urls: vec!["/gif/{*wildcard}".into()], mapping: Mapping::Redirect("http://giphy.com/search/*query*".into()) },

UrlMappings { name: "ukcomp".into(), urls: vec!["/ukcomp/{*wildcard}".into()], mapping: Mapping::Redirect("https://beta.companieshouse.gov.uk/search?q=*query*".into()) },

UrlMappings { name: "unicodes".into(), urls: vec!["/uci/{*wildcard}".into(),"/unicode/{*wildcard}".into()], mapping: Mapping::Redirect("https://apps.timwhitlock.info/unicode/inspect?s=*query*".into()) },

UrlMappings { name: "ucs".into(), urls: vec!["/ucs/{*wildcard}".into(),"/sunicode/{*wildcard}".into()], mapping: Mapping::Redirect("http://www.fileformat.info/info/unicode/char/search.htm?q=*query*&preview=entity".into()) },

UrlMappings { name: "wiki".into(), urls: vec!["/wiki/{*wildcard}".into(),"/enwiki/{*wildcard}".into()], mapping: Mapping::Redirect("https://en.wikipedia.org/w/index.php?search=*query*".into()) },
UrlMappings { name: "german wiki".into(), urls: vec!["/dewiki/{*wildcard}".into()], mapping: Mapping::Redirect("https://de.wikipedia.org/w/index.php?search=*query*".into()) },

UrlMappings { name: "ark".into(), urls: vec!["/ark/{*wildcard}".into()], mapping: Mapping::Redirect("https://ark.intel.com/content/www/us/en/ark/search.html?q=*query*".into()) },
UrlMappings { name: "ansible".into(), urls: vec!["/ansible/{*wildcard}".into()], mapping: Mapping::Redirect("https://docs.ansible.com/projects/ansible/latest/search.html?q=*query*&check_keywords=yes&area=default".into()) },

// ======================
// Static redirects
// ======================
UrlMappings { name: "ula.ext private ipv6".into(), urls: vec!["/ula.ext".into()], mapping: Mapping::Redirect("https://simpledns.plus/private-ipv6".into()) },
UrlMappings { name: "mensa".into(), urls: vec!["/mensa".into()], mapping: Mapping::Redirect("https://www.odr.de/speiseplan/speiseplan.html".into()) },
UrlMappings { name: "bash strings".into(), urls: vec!["/strings.bash".into(),"/bash-strings".into()], mapping: Mapping::Redirect("http://tldp.org/LDP/abs/html/string-manipulation.html".into()) },
UrlMappings { name: "strings.sh".into(), urls: vec!["/strings.sh".into()], mapping: Mapping::Redirect("https://pubs.opengroup.org/onlinepubs/9699919799.2008edition/utilities/V3_chap02.html#tag_18_06_02".into()) },
UrlMappings { name: "fp".into(), urls: vec!["/fp".into()], mapping: Mapping::Redirect("http://i3.kym-cdn.com/photos/images/original/000/001/582/picard-facepalm.jpg".into()) },
UrlMappings { name: "randname".into(), urls: vec!["/randname".into()], mapping: Mapping::Redirect("http://www.behindthename.com/random/random.php?number=1&gender=u&surname=&nodiminutives=yes&all=yes".into()) },


    // ======================
    // Normal routes
    // ======================
    UrlMappings { name: "ip".into(), urls: vec!["/ip".into()], mapping: Mapping::Fn(|| get(|ClientIp(ip)| async move { ip.to_string() })) },
    UrlMappings { name: "ip extended".into(), urls: vec!["/ipi".into()], mapping: Mapping::Fn(|| get(|ClientIp(ip)| ip_data(ip))) },
    UrlMappings { name: "ipi/{*wildcard}".into(), urls: vec!["/ipi/{*wildcard}".into()], mapping: Mapping::Fn(|| get(|Path(e): Path<String>| async move {
        if let Some(ip) = e.to_ip() { ip_data(ip).await } else { "No valid ip given".to_string() }
    }))},
    UrlMappings { name: "base64 encrypt".into(), urls: vec!["/be/{*wildcard}".into()], mapping: Mapping::Fn(|| get(|Path(e): Path<String>| async move { BASE64_STANDARD.encode(e) })) },
    UrlMappings { name: "base64 decrypt".into(), urls: vec!["/bd/{*wildcard}".into()], mapping: Mapping::Fn(|| get(|Path(e): Path<String>| async move {
        BASE64_STANDARD.decode(e).map_or("input is no valid base 64".to_string(), |e| String::from_utf8(e).unwrap())
    }))},
    UrlMappings { name: "url encrypt".into(), urls: vec!["/ue/{*wildcard}".into()], mapping: Mapping::Fn(|| get(|Path(e): Path<String>| async move { urlencoding::encode(&e).into_owned() })) },
    UrlMappings { name: "url decrypt".into(), urls: vec!["/ud/{*wildcard}".into()], mapping: Mapping::Fn(|| get(|Path(e): Path<String>| async move {
        urlencoding::decode(&e).unwrap_or("cant decode url".into()).into_owned()
    }))},
];
let mut wildcards=Vec::new();
let mut functions=Vec::new();
for i in url_mappings {
    match i.mapping{
         Mapping::Redirect(x)=>{
    wildcards.push((i.name,i.urls.clone()));
    for url in i.urls.iter(){
    let value = x.clone();
    app=app.route(url,get(async move |e:Path<_>| redirect_query(e,&value).await));
    }}
     Mapping::Fn(x) =>{
functions.push((i.name,i.urls.clone()));
        for url in i.urls.iter(){
        app=app.route(url,x());
        }
    }
}
}
let mut root_string=String::from("Wildcard redirects:
");
for i in wildcards{
    root_string.push_str(&format!("{}               {}
",i.0,i.1.join(" + ")));
}
root_string.push_str("static redirects:
");
for i in functions{
    root_string.push_str(&format!("{}               {}
",i.0,i.1.join("\n")));
}
app=app.route("/",get(root_string));


//..
    //todo: root
//todo: rfc
//todo: blank posix
//todo: posix
//todo: dhl

app=app.fallback(not_found)
    .layer(config.ip_source.into_extension());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
    Ok(())
}
