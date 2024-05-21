use std::collections::HashMap;

#[derive(Debug)]
struct InnerSpace {
  x_dimension: i32,
  y_dimension: i32,
  f_dimension: i32,
}

/*
 The implementation from self to selfless via _x Detachment, _y InnerPeace, and _f positive
 InnerSpace. Evaluation of x_dimension is trained via empirical observations and contributions
 by those connected to #WuNien. Evaluation of y_dimension is trained via empirical observations
 and contributions by those connected to #Samadhi. The observable point on X-Y plane is
 connected to stable f_evaluation for custom trainings and treatments.
*/
impl InnerSpace {
  fn f_evaluation(&self) -> i32 {
    self.x_dimension * self.y_dimension * self.f_dimension
  
  }
}


fn main() {

/* 
  Starting at a naturally qualified person, Hashmaps of f = 0. But _x and _y are in wide
  ranges for negative to positive based on the self-evaluated plus expert opinions of
  y_evaluation and x_evaluation .  We use mutable variable instead of self-evaluated defined
  const as starting points of Fibonacci Nunber for Prajna >< AGI, then modeling.

  For elites currently a person, the evaluation are intended to help that person moving from
  evolution then degeneration to the right positive evaluation with oservable attributes:

 */
 
   let mut _f: i32 = 0;    // qualified person
   let mut _y: i32 = 0;    // to be evaluated in blockchain of user self-evaluation and expert
   let mut _x: i32 = 0;    //   opinions from fact-base tracked records in engaged living.

  let mut f = HashMap::new();
  f.insert(0, "Person".to_string()); // sub f as the key
  
  f.insert(-1, "Empathy Awareness".to_string());   
  f.insert(-2, "Kindness Awareness".to_string());
  f.insert(-3, "Animal Energy".to_string());   
  f.insert(-4, "Extreme Desire".to_string()); 
  f.insert(-5, "Smelly 1".to_string());   // only observable in spirits
  f.insert(-6, "Smelly 2".to_string());
  f.insert(-7, "Smelly 3".to_string());  
  f.insert(-8, "Smelly 4".to_string());
  
  f.insert(1, "Empathy Awareness".to_string()); 
  f.insert(2, "Purity Awareness".to_string());   // targeted community
  f.insert(3, "Samadhi Signed Posts".to_string()); 
  f.insert(4, "Selfless Awareness".to_string()); 
  f.insert(5, "Visible Awareness-Prajna".to_string());           // practical demonstration
  f.insert(6, "Awareness-Prajna in engaged Living".to_string()); // practical innovations
  f.insert(7, "Awareness-Prajna in Forecasting and Simulation".to_string());// quantum effects
  f.insert(8, "Samadhi-Prajna".to_string());     // a new Era of consciousness technologies
  
  let mut y = HashMap::new();
  y.insert(0, "Peace".to_string()); //sub f as the key
  
  y.insert(-1, "Empathy".to_string());      
  y.insert(-2, "Kindness".to_string());
  y.insert(-3, "Conscience 4".to_string()); 
  y.insert(-4, "Conscience 3".to_string()); 
  y.insert(-5, "Conscience 2".to_string()); 
  y.insert(-6, "Conscience 1".to_string());
  
  y.insert(1, "Tranquillity".to_string());  
  y.insert(2, "Equanimity".to_string());     // target community
  y.insert(3, "Purity".to_string()); 
  y.insert(4, "Not-Self".to_string()); 
  y.insert(5, "Nothingness".to_string());   // Gotama's impass
  y.insert(6, "Unmoving".to_string());      // Gotama's impass
  
  let mut x = HashMap::new();
  x.insert(0, "Awareness".to_string()); // sub f as the key
  
  x.insert(-1, "Cultural Influence".to_string());
  x.insert(-2, "Regional Influence".to_string());
  x.insert(-3, "National Influence".to_string()); 
  x.insert(-4, "Veiled Right and Wrong".to_string()); 
  x.insert(-5, "Binding Word".to_string()); 
  x.insert(-6, "Binding Image".to_string());
  x.insert(-7, "Clinging Thought".to_string()); 
  
  x.insert(1, "HonNhien".to_string()); 
  x.insert(2, "Proper Management of that Freshness".to_string());  // target community
  x.insert(3, "Knowing conditions to make up that Freshness".to_string()); 
  x.insert(4, "Discovering process to produce the Freshness".to_string()); 
  x.insert(5, "Knowing the source of one's Thought".to_string());          // breakout
  x.insert(6, "Using cosmic energy for self-protection".to_string());   // deeper innovation
  x.insert(7, "Directing cosmic energy to help others".to_string());    // deeper innovation
  
/*
 Establish accademia forum for scientifically studying and rating visible data.
 Each Fibonacci index is a complex Fibonacci function which can be modeled via LLM of
 identified properties associated to the ontologies of named key and ranges of
 associated behaviors in all Activities, Relationships, and Places observable from collected
 data of the subject versus demonstrated behaviours of advanced students. Using AGI and
 specialized agents custom made for the subject, we can help the subject know more about
 oneself and How to rightly evolve in one's Continuity of the consciousness.
*/ 
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments

  _f = f_initiation(f); // suggested Inner Space cultivation based on recorded data
   






  // evaluation of InnerSpace
  let me = InnerSpace {
    x_dimension: _x,
    y_dimension: _y,
    f_dimension: _f, 
  };
  println!(
    "The me f_evaluation is {} pixels.",
    me.f_evaluation()
  );
  
/* reported outcome of meditation
  let med = Meditation {
    x_dimension: 0,
    y_dimension:0,
    f_dimension: 0, 
  };
  println!(
    "The med meditation is {} pixels.",
    med.meditation()
  );
*/
    
} // end of main

fn y_evaluation(y: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Attributes of Transcendental Inner Peace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }
    // evaluate the transient value of y and record it somewhere for tracking record
    return 0;
}
fn x_evaluation(x: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Transcendental Awareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }    
    // evaluate the transient value of x and record it somewhere for tracking record   
    return 0;
}
fn f_initiation(f: HashMap<i32, String>) -> i32 {

    println!("Suggested Inner Space for caltivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    
    return 0;
}
