extern crate time;

static N_GRID_CAPITAL: uint = 17820;
static N_GRID_PRODUCTIVITY: uint = 5;
  

fn do_stuff() {

  /////////////////////////////////////////////////////////////////////
  // 1. Calibration
  /////////////////////////////////////////////////////////////////////

  let aalpha = 1f64/3.0;
  let bbeta = 0.95f64;

  let v_productivity = [0.9792f64, 0.9896, 1.0000, 1.0106, 1.0212];

  let m_transition = [
    [0.9727, 0.0273, 0.0000, 0.0000, 0.0000],
    [0.0041, 0.9806, 0.0153, 0.0000, 0.0000],
    [0.0000, 0.0082, 0.9837, 0.0082, 0.0000],
    [0.0000, 0.0000, 0.0153, 0.9806, 0.0041],
    [0.0000, 0.0000, 0.0000, 0.0273, 0.9727]
  ];

  /////////////////////////////////////////////////////////////////////
  // 2. Steady State
  /////////////////////////////////////////////////////////////////////

  let capital_steady_state = (aalpha*bbeta).powf(1.0/(1.0-aalpha));
  let output_steady_state = capital_steady_state.powf(aalpha);
  let consumtion_steady_state = output_steady_state - capital_steady_state;

  println!("Output = {:.17f}, Capital = {:.17f}, Consumption = {:.17f}",
    output_steady_state,
    capital_steady_state,
    consumtion_steady_state);

  let n_grid_capital = N_GRID_CAPITAL;
  let n_grid_productivity = N_GRID_PRODUCTIVITY;
  
  let mut v_grid_capital = [0f64,..N_GRID_CAPITAL];
  for n_capital in range(0, N_GRID_CAPITAL) {
    v_grid_capital[n_capital] = 0.5 * capital_steady_state + 0.00001 * (n_capital as f64);
  }
  let v_grid_capital = v_grid_capital;

  /////////////////////////////////////////////////////////////////////
  // 3. Required matrices and vectors
  /////////////////////////////////////////////////////////////////////

  let mut m_output = box () ([[0f64, ..N_GRID_PRODUCTIVITY], ..N_GRID_CAPITAL]);
  let mut m_value_function = box () ([[0f64, ..N_GRID_PRODUCTIVITY], ..N_GRID_CAPITAL]);
  let mut m_value_function_new = box () ([[0f64, ..N_GRID_PRODUCTIVITY], ..N_GRID_CAPITAL]);
  let mut m_policy_function = box () ([[0f64, ..N_GRID_PRODUCTIVITY], ..N_GRID_CAPITAL]);
  let mut expected_value_function = box () ([[0f64, ..N_GRID_PRODUCTIVITY], ..N_GRID_CAPITAL]);

  /////////////////////////////////////////////////////////////////////
  // 4. We pre-build output for each point in the grid
  /////////////////////////////////////////////////////////////////////

  for n_productivity in range(0, N_GRID_PRODUCTIVITY) {
    for n_capital in range(0, N_GRID_CAPITAL) {
      m_output[n_capital][n_productivity] =
        v_productivity[n_productivity] * v_grid_capital[n_capital].powf(aalpha);
    }
  }
  let m_output = m_output;

  /////////////////////////////////////////////////////////////////////
  // 5. Main iteration
  /////////////////////////////////////////////////////////////////////

  let mut max_difference = 10.0f64;
  let tolerance = 0.0000001f64;

  let mut iteration = 0;

  while max_difference > tolerance {
    for n_productivity in range(0,N_GRID_PRODUCTIVITY) {
      for n_capital in range(0, N_GRID_CAPITAL) {
        expected_value_function[n_capital][n_productivity] = 0.0;
        for n_productivity_next_period in range(0, N_GRID_PRODUCTIVITY) {
          expected_value_function[n_capital][n_productivity] +=
            m_transition[n_productivity][n_productivity_next_period] * m_value_function[n_capital][n_productivity_next_period];
        }
      }
    }

    for n_productivity in range(0, N_GRID_PRODUCTIVITY) {
      let mut grid_capital_next_period = 0;

      for n_capital in range(0, N_GRID_CAPITAL) {

        let mut value_high_sofar = -100000.0;

        for n_capital_next_period in range(grid_capital_next_period, N_GRID_CAPITAL) {
          let consumption = m_output[n_capital][n_productivity] - v_grid_capital[n_capital_next_period];
          let value_provisional = (1.0-bbeta) * consumption.ln() + bbeta *
              expected_value_function[n_capital_next_period][n_productivity];

          if value_provisional > value_high_sofar {
            value_high_sofar = value_provisional;
            let capital_choice = v_grid_capital[n_capital_next_period];
            grid_capital_next_period = n_capital_next_period;
            m_value_function_new[n_capital][n_productivity] = value_high_sofar;
            m_policy_function[n_capital][n_productivity] = capital_choice;
          } else {
            break;
          }
        }
      }
    }

    let mut diff_high_sofar = -100000.0f64;
    for n_productivity in range(0, n_grid_productivity) {
      for n_capital in range(0, n_grid_capital) {
        let diff = (m_value_function[n_capital][n_productivity] -
            m_value_function_new[n_capital][n_productivity]).abs();
        diff_high_sofar = diff_high_sofar.max(diff);
        m_value_function[n_capital][n_productivity] = m_value_function_new[n_capital][n_productivity];
      }
    }
    max_difference = diff_high_sofar;

    iteration += 1;
    if iteration % 10 == 0 || iteration == 1 {
      println!("Iteration = {}, Sup Diff = {}", iteration, max_difference);
    }
  }
  println!("Iteration = {}, Sup Diff = {}", iteration, max_difference);
  println!("My check = {}", m_policy_function[999][2]);
}

fn main() {
  let start = ::time::precise_time_ns() as f64;

  do_stuff();

  let end = ::time::precise_time_ns() as f64;
  let time_difference = end - start;
  let nano_seconds = 1000000000.0;
  println!("Elapsed time is {} seconds.", time_difference / nano_seconds);
}
