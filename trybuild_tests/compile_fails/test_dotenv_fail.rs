fn main() {
    use typenum_consts::tnconst;

    type A = tnconst![+ env!("SEXY_SECRET");];
    type F = tnconst![+ env!("SEXY_SECRET", "");];
    type B = tnconst![+ env!("SEXY_SECRET",,);];
    type C = tnconst![+ env!(,,"SEXY_SECRET");];
    type D = tnconst![+ env!("", );];
    type E = tnconst![+ env!("", "");];
}
