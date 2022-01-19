// let staff = [
//   { name: "George", money: 0 },
//   { name: "Lea", money: 500000 },
// ];
// let salary = 1000;
// staff.forEach((employee) => {
//   employee.money += salary;
// });
#[derive(Debug)]
pub struct ContextArray {
   pub name: String,
   pub money: i32,
}

fn main() {
    let mut statff= vec![
        ContextArray { name: "George".to_string(), money: 0},
        ContextArray { name: "Lea".to_string(), money: 1000}
    ];
    let salary = 1000;

    statff.iter_mut().for_each(
        |item| item.money += salary
    );
    println!("{:?}", statff)
}
