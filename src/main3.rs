use quiche;
use quiche::h3::NameValue;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn f1() -> quiche::Connection {
    println!("Hello, world!");
    let scid = quiche::ConnectionId::from_vec(vec![1]);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
    config.set_application_protos(quiche::h3::APPLICATION_PROTOCOL);
    let mut conn = quiche::accept(&scid, None, socket, &mut config).unwrap();

    let mut h3_config = quiche::h3::Config::new().unwrap();
    let mut h3_conn = quiche::h3::Connection::with_transport(&mut conn, &h3_config).unwrap();

    loop {
        match h3_conn.poll(&mut conn) {
            Ok((stream_id, quiche::h3::Event::Headers { list, has_body })) => {
                let mut headers = list.into_iter();

                // Look for the request's method.
                let method = headers.find(|h| h.name() == b":method").unwrap();

                // Look for the request's path.
                let path = headers.find(|h| h.name() == b":path").unwrap();

                if method.value() == b"GET" && path.value() == b"/" {
                    let resp = vec![
                        quiche::h3::Header::new(b":status", 200.to_string().as_bytes()),
                        quiche::h3::Header::new(b"server", b"quiche"),
                    ];

                    h3_conn.send_response(&mut conn, stream_id, &resp, false);
                    h3_conn.send_body(&mut conn, stream_id, b"Hello World!", true);
                }
            }

            Ok((stream_id, quiche::h3::Event::Data)) => {
                // Request body data, handle it.
            }

            Ok((stream_id, quiche::h3::Event::Finished)) => {
                // Peer terminated stream, handle it.
            }

            Ok((stream_id, quiche::h3::Event::Reset(err))) => {
                // Peer reset the stream, handle it.
            }

            Ok((_flow_id, quiche::h3::Event::Datagram)) => (),

            Ok((_flow_id, quiche::h3::Event::PriorityUpdate)) => (),

            Ok((goaway_id, quiche::h3::Event::GoAway)) => {
                // Peer signalled it is going away, handle it.
            }

            Err(quiche::h3::Error::Done) => {
                // Done reading.
                break;
            }

            Err(e) => {
                // An error occurred, handle it.
                break;
            }
        }
    }

    return conn;
}

fn main() {
    println!("Hello World!");
    f1();
    println!("Hello World3 !");
}
