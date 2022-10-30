const SECONDS_IN_MINUTE: u32 = 60; // precisa definir tipo explicitamente e imutavel

fn main() { // escopo
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;
    // let _total = 30; _ para ignorar coisas, nesse caso uma variavel nao usavel
    // let total: i32 = 30; quando o compilador nao consegue inferir, fazemos manualmente
    // let total = 30; // add mut para a variavel ser mutavel
    // println!("Trabalhou {} horas", total);
    // total = 44; 
    // let total = "quarenta";
    // add let se a var iniciou int e quer mudar para outro obj e nao precisa do mut no começo
    // println!("Trabalhou {} horas", total);
    // subescopo
    // let total = 30;
    // {
    //     let total = total + 20; 
    //     println!("Trabalhou {} horas", total);
    // }
    // println!("Trabalhou {} horas", total);
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} s", total_em_segundos);

} // fim
// drop
