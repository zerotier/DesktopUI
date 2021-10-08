// This is an adaptation of an example by Microsoft, using WinRT APIs to open a file and read bytes from it.
// Original source: https://github.com/Microsoft/Windows-universal-samples/blob/ad44f9b846b7064a0dd6eca8b0b7a453902e4a22/Samples/DataReaderWriter/cs/Scenario2_ReadBytes.xaml.cs
//
// Use the following command to run this example:
// > cargo run --example hexdump --features "windows-storage"

extern crate winrt;

use winrt::*;
use winrt::windows::foundation::*;
use winrt::windows::storage::*;

fn main() {
    let rt = RuntimeContext::init();
    run();
    rt.uninit();
}

const BYTES_PER_ROW: usize = 24;
const CHUNK_SIZE: usize = 4096;

fn run() { unsafe {
    // Use the current executable as source file (because we know that will exist).
    let exe_path = ::std::env::current_exe().expect("current_exe failed");
    let exe_path_str = exe_path.to_str().expect("invalid unicode path");

    let file = StorageFile::get_file_from_path_async(&*FastHString::new(&exe_path_str)).unwrap().blocking_get().expect("get_file_from_path_async failed");
    println!("Dumping file: {}", file.query_interface::<IStorageItem>().unwrap().get_path().unwrap());

    // Open a sequential-access stream over the file.
    let input_stream = file.query_interface::<streams::IInputStreamReference>().unwrap().open_sequential_read_async().unwrap().blocking_get().unwrap();
    // Pass the input stream to the DataReader.
    let data_reader = streams::DataReader::create_data_reader(&input_stream).unwrap();
    let mut curr_chunk = 0;
    let mut bytes = [0u8; BYTES_PER_ROW];
    
    loop {
        // Load the next chunk into the DataReader buffer.
        let num_bytes = data_reader.load_async(CHUNK_SIZE as u32).expect("load").blocking_get().expect("get") as usize;

        let mut num_bytes_remaining = num_bytes;
        while num_bytes_remaining >= BYTES_PER_ROW
        {
            // Use the DataReader and read_bytes() to fill the byte array with one row worth of bytes.
            data_reader.read_bytes(&mut bytes).unwrap();
            print_row(&bytes, (num_bytes - num_bytes_remaining) + (curr_chunk * CHUNK_SIZE));
            num_bytes_remaining -= BYTES_PER_ROW;
        }

        // If there are any bytes remaining to be read, allocate a new array that will hold
        // the remaining bytes read from the DataReader and print the final row.
        // Note: read_bytes() fills the entire array so if the array being passed in is larger
        // than what's remaining in the DataReader buffer, an error will be returned.
        if num_bytes_remaining > 0
        {
            // Use the DataReader and read_bytes() to fill the byte array with the last row worth of bytes.
            data_reader.read_bytes(&mut bytes[0..num_bytes_remaining]).unwrap();

            print_row(&bytes[0..num_bytes_remaining], (num_bytes - num_bytes_remaining) + (curr_chunk * CHUNK_SIZE));
        }

        curr_chunk += 1;

        // If the number of bytes read is anything but the chunk size, then we've just retrieved the last
        // chunk of data from the stream. Otherwise, keep loading data into the DataReader buffer.
        if num_bytes != CHUNK_SIZE {
            break;
        }
    }

    data_reader.query_interface::<IClosable>().unwrap().close().unwrap();
    input_stream.query_interface::<IClosable>().unwrap().close().unwrap();
} }

fn print_row(bytes: &[u8], curr_byte: usize) {
    // Format the address of byte i to have 8 hexadecimal digits and add the address
    // of the current byte to the output string.
    print!("{:08X}:", curr_byte);

    // Format the output:
    for (i, byte) in bytes.iter().enumerate() {

        // If finished with a segment, add a space.
        if i % 2 == 0 {
            print!(" ");
        }

        // Convert the current byte value to hex and add it to the output string.
        print!("{:02X}", byte);
    }

    println!("");
}
