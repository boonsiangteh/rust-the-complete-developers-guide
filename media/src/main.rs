#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie { title: String, director: String },
    Audiobook { title: String},
    Podcast(u32),
    Placeholder,
}

#[derive(Debug)]
struct  Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog {items: vec![]}
    }

    fn addItems(&mut self,  media: Media) {
        self.items.push(media);
    }
}


impl Media {
    fn description(&self) -> String {
        if let Media::Book {title, author} = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director} = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }

    }
}

fn main() {
    
    let book = Media::Book { title: String::from("a book"), author: String::from("book author") };
    let movie = Media::Movie { title: String::from("a movie"), director: String::from("john") };
    let audiobook = Media::Audiobook { title: String::from("an audiobook") } ;
    let podcast = Media::Podcast(32);
    let placeholder = Media::Placeholder;


    // println!("{:#?}", book.description());
    // println!("{:#?}", movie.description());
    // println!("{:#?}", podcast.description());
    // println!("{:#?}", placeholder.description());

    let mut catalog = Catalog::new();
    // println!("{:#?}", catalog);
    catalog.addItems(book);
    catalog.addItems(movie);
    catalog.addItems(audiobook);
    catalog.addItems(placeholder);
    catalog.addItems(podcast);
    println!("{:#?}", catalog);
}
