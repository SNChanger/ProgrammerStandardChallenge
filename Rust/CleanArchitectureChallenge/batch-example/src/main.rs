use application::presenters::add_user_presenter::AddUserPresenter;
use business::add_user_case::add_user_input_message::AddUserInputMessage;
use business::add_user_case::add_user_interacotr::AddUserInteractor;
use business::InputBoundary;
use infrastructure::repositories::file_repository::FileRepository;


fn main() {
    let input_message = init_input_message();
    let presenter = Box::new(AddUserPresenter::new());

    let mut interactor = AddUserInteractor::new(presenter, init_file_repository());

    interactor.execute(input_message);
}


fn init_input_message() -> AddUserInputMessage {
    let last_name = String::from("Last Name Example");
    let first_name = String::from("First Name Example");
    let emal = String::from("email@example.com");

    AddUserInputMessage::new(last_name, first_name, email)

}

fn init_file_repository() -> Box<FileRepository> {
    let file_path = String::from("tmp/example.com");
    
}

