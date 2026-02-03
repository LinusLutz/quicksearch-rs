use axum::Router;
use axum::extract::{ConnectInfo, Path};
use axum::response::Redirect;
use axum::routing::get;
use std::net::SocketAddr;

async fn redirect_query(Path(query): Path<String>, redirect_url: &str) -> Redirect {
    Redirect::to(&redirect_url.replace("*query*", &query))
}
async fn redirect(redirect_url: &str) -> Redirect {
    Redirect::to(&redirect_url)
}
//async fn trains(query: String)->String {
//    let UIC_COUNTRY_CODES = [("FI", 10), ("RU", 20),("BY", 21),("UA", 22),("MD", 23),("LT", 24),("LV", 25),("EE", 26),("KZ", 27),("GE", 28),("UZ", 29),("KP", 30),("MN", 31),("VN", 32),("CN", 33),("LA", 34),("CU", 40),("AL", 41),("JP", 42),("BA", 44),("BA", 49),("BA", 50),("PL", 51),("BG", 52),("RO", 53),("CZ", 54),("HU", 55),("SK", 56),("AZ", 57),("AM", 58),("KG", 59),("IE", 60),("KR", 61),("ME", 62),("MK", 65),("TJ", 66),("TM", 67),("AF", 68),("GB", 70),("ES", 71),("RS", 72),("GR", 73),("SE", 74),("TR", 75),("NO", 76),("HR", 78),("SI", 79),("DE", 80),("AT", 81),("LU", 82),("IT", 83),("NL", 84),("CH", 85),("DK", 86),("FR", 87),("BE", 88),("TZ", 89),("EG", 90),("TN", 91),("DZ", 92),("MA", 93),("PT", 94),("IL", 95),("IR", 96),("SY", 97),("LB", 98),("IQ", 99)];
//
//}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    let app = Router::new()
    .route("/ip", get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { addr.ip().to_string() }))
    
    //..
    //todo: root

    //todo: urlencode/urldecode
    .route("/ula.ext", get(redirect("https://simpledns.plus/private-ipv6").await))
    .route("/mensa", get(redirect("http://www.stwno.de/infomax/daten-extern/html/speiseplaene.php?einrichtung=UNI-P").await))
    .route("/strings.bash", get(redirect("http://tldp.org/LDP/abs/html/string-manipulation.html").await))
    .route("/bash-strings", get(redirect("http://tldp.org/LDP/abs/html/string-manipulation.html").await))
    .route("/strings.sh", get( redirect("https://pubs.opengroup.org/onlinepubs/9699919799.2008edition/utilities/V3_chap02.html#tag_18_06_02").await))
    .route("/fp", get( redirect("http://i3.kym-cdn.com/photos/images/original/000/001/582/picard-facepalm.jpg").await))
    .route("/randname", get( redirect("http://www.behindthename.com/random/random.php?number=1&gender=u&surname=&nodiminutives=yes&all=yes").await))

    .route("/i/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*&tbm=isch")))
    .route("/gi/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*&tbm=isch")))

    .route("/gv/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*&tbm=vid")))
    .route("/v/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*&tbm=vid")))

    .route("/google/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*")))
    .route("/g/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.google.com/search?q=*query*")))

    .route("/nixo/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://search.nixos.org/options?query=*query*")))
    .route("/nixp/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://search.nixos.org/packages?query=*query*")))

    .route("/inwx/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.inwx.de/de/domain/check#search=*query*#region=DEFAULT#rc=rc1")))
    .route("/ovh/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.ovh.de/cgi-bin/newOrder/order.cgi?domain_domainChooser_domain=*query*")))
    .route("/tineye/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://tineye.com/search?url=*query*")))
    
    .route("/madison/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://qa.debian.org/madison.php?table=all&g=on&package=*query*")))
    .route("/deb/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://qa.debian.org/madison.php?table=debian&g=on&package=*query")))
    .route("/ubu/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://qa.debian.org/madison.php?table=ubuntu&g=on&package=*query*")))
    .route("/dpkg/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://packages.debian.org/search?keywords=*query*")))
    .route("/upkg/{*wildcard}", get(|e:Path<_>| redirect_query(e,"http://packages.ubuntu.com/search?keywords=*query*")))
    .route("/apkg/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.archlinux.org/packages/?q=*query*")))
    .route("/aur/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://aur.archlinux.org/packages/?K=*query*")))
    .route("/repo/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://repology.org/projects/?search=*query*")))
    //todo: repo blank

    .route("/fport/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.freshports.org/search.php?num=20&query=*query*")))
    .route("/fports/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.freshports.org/search.php?num=20&query=*query*")))
    .route("/freshports/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.freshports.org/search.php?num=20&query=*query*")))

    .route("/gpkg/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://packages.gentoo.org/packages/search?q=*query*")))
    .route("/eix/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://packages.gentoo.org/packages/search?q=*query*")))
    
    .route("/denic/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.denic.de/webwhois-web20/?domain=*query*")))
    .route("/ssll/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.ssllabs.com/ssltest/analyze.html?d=*query*&hideResults=on&latest")))
    .route("/bgp/{*wildcard}", get(|e:Path<_>| redirect_query(e,"http://bgp.he.net/search?commit=Search&search[search]=*query*")))
    .route("/tld/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://tld-list.com/tld/*query*")))
    .route("/woa/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.wolframalpha.com/input/?i=*query*")))

    .route("/dcc/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://www.dict.cc/?s=*query*")))
    
    .route("/gif/{*wildcard}", get(|e:Path<_>| redirect_query(e,"http://giphy.com/search/*query*")))

    .route("/ukcomp/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://beta.companieshouse.gov.uk/search?q=*query*")))

    .route("/uci/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://apps.timwhitlock.info/unicode/inspect?s=*query*")))
    .route("/unicode/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://apps.timwhitlock.info/unicode/inspect?s=*query*")))
    
    .route("/ucs/{*wildcard}", get(|e:Path<_>| redirect_query(e,"http://www.fileformat.info/info/unicode/char/search.htm?q=*query*&preview=entity")))
    .route("/sunicode/{*wildcard}", get(|e:Path<_>| redirect_query(e,"http://www.fileformat.info/info/unicode/char/search.htm?q=*query*&preview=entity")))

    .route("/wiki/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://en.wikipedia.org/w/index.php?search=*query*")))
    .route("/enwiki/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://en.wikipedia.org/w/index.php?search=*query*")))
    .route("/dewiki/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://de.wikipedia.org/w/index.php?search=*query*")))
//todo: rfc
    .route("/ark/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://ark.intel.com/content/www/us/en/ark/search.html?q=%s")))
//todo: blank posix
//todo: posix
//todo: dhl

    .route("/ansible/{*wildcard}", get(|e:Path<_>| redirect_query(e,"https://docs.ansible.com/projects/ansible/latest/search.html?q=*query*&check_keywords=yes&area=default")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
    Ok(())
}
