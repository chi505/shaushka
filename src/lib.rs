use youtube_dl::YoutubeDl;
use serde::Deserialize;

pub async fn process_urls(urls: &Vec<String>) -> String {
    for url in urls {
        let result = YoutubeDl::new(url).socket_timeout("60").extract_audio(true).extra_arg("--audio-format").extra_arg("mp3").run();
        match result{
            Ok(result) =>    match result {
                youtube_dl::YoutubeDlOutput::Playlist(_playlist) => {return (*_playlist).entries.unwrap()[0].description.as_ref().unwrap().to_string() },
                youtube_dl::YoutubeDlOutput::SingleVideo(_single_video) => {return (*_single_video).description.unwrap().to_string()}
            },
            Err(err) => println!("{}",err),
        }
     
    }
    return "".to_string()
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Directive {
    pub stream_index: u32,
    pub start_sample: u32,
    pub start_insert: u32,
}

#[derive(Deserialize)]
pub struct TrackAssemblyInfo {
    pub urls: Vec<String>,
    pub directives: Vec<Directive>
}

pub async fn enact_directive(directive: &Directive){
    println!("{},{},{}", directive.stream_index, directive.start_sample, directive.start_insert)
}