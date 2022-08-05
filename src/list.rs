use handlebars::{handlebars_helper, Handlebars};

pub fn addhelpers(x: &mut Handlebars) {
    handlebars_helper!(until_step: |start: isize, max: isize, step: usize| {
        let mut arr: Vec<isize> = Vec::new();
        for num in (start..max).step_by(step){
            arr.push(num);
        }
        arr
    });
    handlebars_helper!(seq: | args: Vec<isize> |{
        let mut arr: Vec<isize> = Vec::new();
        if args.len() == 1 {
            for num in 1..args[0] {
                arr.push(num);
            }
        }  else if args.len() == 2 {
            for num in args[0]..args[1] {
                arr.push(num);
            }
        } else if args.len() == 3 {
            let mut num = args[0];

            while  if args[1] < 0 {num > args[2]} else {num < args[2]} {
                arr.push(num);
                num += args[1];
            }
        }
        arr
    });

    x.register_helper("until_step", Box::new(until_step));
    x.register_helper("seq", Box::new(seq));
}
