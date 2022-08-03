use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;


#[derive(Clone)]
struct Movie {
    title: String,
    submitted_by: String,
    score: u32,
}

impl Movie {
    fn update_score(&mut self, new_score: u32) {
        self.score = self.score + new_score;
    }
}

fn print_many_lines(number_of_lines: u32) {
    for _ in 0..number_of_lines {
        println!("\n");
    }
}

fn get_list_of_movies() -> Vec<Movie> {
    let mut movies:Vec<Movie> = Vec::new();
    loop {
        let mut submitted_by = String::new();
        let mut title = String::new();
        let mut more_users = String::new();
        print!("What's your name?: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut submitted_by).expect("Failed to read line");
        print!("What do you want to watch (title of Documentary / Movie etc)?: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut title).expect("Failed to read line");
        let movie = Movie {
            title: title.trim().to_string(),
            submitted_by: submitted_by.trim().to_string(),
            score: 0,
        };
        movies.push(movie);
        print!("Is there any one else selecting a movie?(y/n): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut more_users).expect("Failed to read line");
        if more_users.trim() == "n" {
            break;
        }

    }
    movies
}

fn validate_score(score: &str, lower_bound:u32, upper_bound:u32) -> bool {
    let score_int: u32 = score.trim().parse().unwrap_or(0);
    if score_int >= lower_bound && score_int <= upper_bound {
        return true;
    }
    false
}

fn tie_breaker(movies: Vec<Movie>) {
    let movie_list = movies.clone();
    let mut top_movies:Vec<Movie> = Vec::new();
    let mut max_score:u32 = 0;
    for movie in &movies {
        if movie.score > max_score {
            max_score = movie.score;
        }
    }

    for movie in movies {
        if movie.score == max_score {
            top_movies.push(movie);
        }
    }
    if top_movies.len() > 1 {
        for movie in movie_list {
            print_many_lines(20);
            println!("***There is a tie!***\n Time for ELIMINATION!\n");
            println!("***{}*** please come for a secret vote!", movie.submitted_by);
            let mut movie_names:Vec<String> = Vec::new();
            for top_movie in &top_movies {
                movie_names.push(top_movie.title.clone());
            }
            loop {
                let mut user_choice = String::new();
                print!("Select *one* movie from the following list: {:?}, if you select the 1st one than type 1, for the 2nd one type 2 and so on..:", movie_names);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut user_choice).expect("Failed to read line");
                if validate_score(&user_choice, 1, top_movies.len() as u32) {
                    let mut user_choice_int: u32 = user_choice.trim().parse().unwrap();
                    user_choice_int = user_choice_int - 1;
                    let selected_movie = &mut top_movies[user_choice_int as usize];
                    selected_movie.update_score(1);
                    break;
                }
            
            }
            
        }
        tie_breaker(top_movies);
}
else {
    println!("The winner is: {} Submitted by: {}!", top_movies[0].title, top_movies[0].submitted_by);
}
}

fn score_movies(movies: Vec<Movie>) -> Vec<Movie> {
    let mut random_movie_list:Vec<Movie> = movies.to_vec();
    let mut rng = thread_rng();
    random_movie_list.shuffle(&mut rng);
    for movie in movies {
        print_many_lines(20);
        println!("***{}*** please come for a secret vote!", movie.submitted_by);
        for random_movie in &mut random_movie_list {
            loop {
                let mut score = String::new();
                println!("Score {} on a scale of 1 to 3, 1 being the worst and 3 being the best: ", random_movie.title);
                io::stdin().read_line(&mut score).expect("Failed to read line");
                if validate_score(&score, 1, 3) {
                    random_movie.update_score(score.trim().parse().unwrap());
                    break;
                } else {
                    println!("Invalid score, please try again");
                }
            }
        }

    }
    random_movie_list 
}


fn main() {

    let movies:Vec<Movie> = get_list_of_movies();
    let scored_movies:Vec<Movie> = score_movies(movies);
    tie_breaker(scored_movies);


  
}

