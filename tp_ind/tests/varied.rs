mod common;

#[cfg(test)]
mod varied {
    use crate::common;

    use common::{comparar_resultado_stack, escribir_en_archivo};

    #[test]
    fn unit_computation_1() -> Result<(), String> {
        escribir_en_archivo(
            ": meter 100 * ;
      : decimeter 10 * ;
      : centimeter 1 * ;
      1 meter 5 decimeter 2 centimeter + +
      ",
        )?;
        comparar_resultado_stack(vec![152])
    }

    #[test]
    fn unit_computation_2() -> Result<(), String> {
        escribir_en_archivo(
            ": seconds 1 * ;
      : minutes 60 * seconds ;
      : hours 60 * minutes ;
      2 hours 13 minutes 5 seconds + +
      ",
        )?;
        comparar_resultado_stack(vec![7985])
    }

    #[test]
    fn constant_sumation() -> Result<(), String> {
        escribir_en_archivo(
            ": one1 1 ;
      : one2  one1 one1 ;
      : one4  one2 one2 ;
      : one8  one4 one4 ;
      : one16 one8 one8 ;
      : add1 + ;
      : add2  add1 add1 ;
      : add4  add2 add2 ;
      : add8  add4 add4 ;
      : add16 add8 add8 ;
      0
      one16
      add16
      ",
        )?;
        comparar_resultado_stack(vec![16])
    }

    #[test]
    fn linner_summation() -> Result<(), String> {
        escribir_en_archivo(
            ": next1 dup 1 + ;
      : next2  next1 next1 ;
      : next4  next2 next2 ;
      : next8  next4 next4 ;
      : next16 next8 next8 ;
      : add1 + ;
      : add2  add1 add1 ;
      : add4  add2 add2 ;
      : add8  add4 add4 ;
      : add16 add8 add8 ;
      0
      next16
      add16
      ",
        )?;
        comparar_resultado_stack(vec![136])
    }

    #[test]
    fn geometric_summation() -> Result<(), String> {
        escribir_en_archivo(
            ": next1 dup 2 * ;
      : next2  next1 next1 ;
      : next4  next2 next2 ;
      : next8  next4 next4 ;
      : add1 + ;
      : add2  add1 add1 ;
      : add4  add2 add2 ;
      : add8  add4 add4 ;
      1
      next8
      add8
      ",
        )?;
        comparar_resultado_stack(vec![511])
    }

    #[test]
    fn part_of_2() -> Result<(), String> {
        escribir_en_archivo(
            ": next1 dup 2 * ;
      : next2  next1 next1 ;
      : next4  next2 next2 ;
      : mul1 * ;
      : mul2  mul1 mul1 ;
      : mul4  mul2 mul2 ;
      1
      next4
      mul4
      ",
        )?;
        comparar_resultado_stack(vec![1024])
    }

    #[test]
    #[ignore = "Implemenacion de test por lectura de consola en proceso"]
    fn digit_to_string() -> Result<(), String> {
        escribir_en_archivo(
            ": f
        dup 0 = if
          drop .' zero'
        else dup 1 = if
          drop .' one'
        else dup 2 = if
          drop .' two'
        then then then ;
      0 f cr
      1 f cr
      2 f cr
      ",
        )?;
        comparar_resultado_stack(vec![])
    }
}
