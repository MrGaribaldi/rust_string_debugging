use std::error::Error;
use std::time::Instant;

static NUM_PARAGRAPHS: i32 = 1500;

struct CustomResult {
    read_string:String
}

fn get_string_as_custom_result() -> CustomResult {
    let mut ret_val = CustomResult { read_string: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n") };

    for _ in 0..NUM_PARAGRAPHS {
        String::push_str(&mut ret_val.read_string, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n");
    }

    let _break_here_to_trigger_minimal_delay = 0;
    println!("{:?}", Instant::now());
    ret_val
}

fn get_string_as_result() -> Result<String, Box<dyn Error>> {
    let mut contents: String = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n");
    for _ in 0..NUM_PARAGRAPHS {
        String::push_str(&mut contents, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n")
    }
    let _break_here_to_trigger_delay = 0;
    println!("{:?}", Instant::now());
    Ok(contents)
}

fn get_string() -> String {
    let mut contents: String = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n");
    for _ in 0..NUM_PARAGRAPHS {
        String::push_str(&mut contents, "{} Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Eget velit aliquet sagittis id consectetur. Tincidunt id aliquet risus feugiat in ante metus dictum. Porta nibh venenatis cras sed felis eget. A arcu cursus vitae congue mauris rhoncus aenean vel. Scelerisque mauris pellentesque pulvinar pellentesque habitant morbi tristique. Ornare arcu odio ut sem nulla pharetra. Praesent elementum facilisis leo vel fringilla est ullamcorper eget. Nisi est sit amet facilisis magna etiam tempor orci eu. Tincidunt vitae semper quis lectus nulla at. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Et odio pellentesque diam volutpat commodo sed. Ac felis donec et odio pellentesque diam volutpat commodo sed. Nunc faucibus a pellentesque sit amet porttitor eget dolor. Pretium quam vulputate dignissim suspendisse in est. Odio morbi quis commodo odio aenean sed adipiscing diam.\n")
    }
    let _break_here_to_trigger_delay = 0;
    println!("{:?}", Instant::now());

    contents
}

fn main() {

    {
        println!("get_string()");
        let _string_contents = get_string();
        println!("{:?}", Instant::now());
        //resume program here
    }
    {
        println!("get_string_as_custom_result()");
        let _custom_result = get_string_as_custom_result();
        println!("{:?}", Instant::now());
        //resume program here
    }
    {

        println!("get_string_as_result()");
        let _result_contents = get_string_as_result();
        println!("{:?}", Instant::now());

    }


}
