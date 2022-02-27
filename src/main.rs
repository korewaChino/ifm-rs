use std::net::*;
/// iFacialMocap/Facemotion3D data parsing algorithm, ported from VSS (GDScript)
/// This script captures packets from an iFacialMocap instance and pretty-prints them to the console.
fn main() {
    // Listen to UDP packets on port 49983
    let socket = UdpSocket::bind("0.0.0.0:49983").unwrap();
    println!("Listening on port 49983");
    // then print out the contents of each packet received
    loop {
        let mut buf = [0; 1024];
        let _packet = socket.recv_from(&mut buf).unwrap();
        // print the text in the packet
        let data = String::from_utf8_lossy(&buf);
        // The data is a massive string, which is an array of tracking data seperated by |
        // We want to split each key/value pair with | and then split the key and value with -
        // There's a special case for XYZ coordinates, which the key and value is separated by #

        // I know this implementation is ugly, but it's just a quick and dirty way to get the data.
        // According to the Rust Discord, I should've used if let Some = text.split_once("-"){} to split the key and value
        // But this is simply a logic port from my old GDScript code

        // Split the data into key/value pairs, then if it's a XYZ coordinate, split the key as an array
        data.split("|").for_each(|pair| {
            // then check if there's a # in the pair
            if pair.contains("#") {
                // if there is, split the pair by #
                let mut pair = pair.split_once("#");
                // this returns a tuple of (key, value)
                let (key, value) = pair.unwrap();

                // split the coords by , and make it an array of floats
                let coords: Vec<f32> = value.split(",").map(|x| x.parse().unwrap()).collect();
                // print out the key and the coords
                println!("{}: {:?}", key, coords);
            }
            else if pair.contains("-") {
                // if there isn't, split the pair by -
                let mut pair = pair.split_once("-");
                // this returns a tuple of (key, value)
                let (key, value) = pair.unwrap();
                // print out the key and value
                println!("{}: {}", key, value);
            }
        });
    }
}
