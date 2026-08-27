//! Punto de entrada del proyecto.
//! Cada archivo se encarga de una tarea específica para que sea fácil de estudiar.

mod calculadora;
mod ecuaciones;
mod servidor;

fn main() {
    servidor::iniciar();
}
