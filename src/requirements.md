requirements:
  - easy way to define sequences with 2 types:
    1. short running: any unexpected key would cause the sequence to reset
    2. long running: a specific key to reset

    a { b { c }}

    d==0 && a {
        d=1
    }
    d==1 && b {
        d=2
    }
    d==2 && c {
        // ...
        d=0
    }

    esc {
        d=0
    }

  - simple easy to hack event handling:

        asd: bool

        fn setup() {
            asd = false;
        }

        fn on_event(e) {
            if e == KeyPress(Alt_L) {
                asd = true;
            }
            if e == KeyRelease(Alt_L) {
                asd = true;
            }
        }


        // or:

        fn start() {

            let asd = asd;

            while let Some(event) = wait_for_event(e) {
                if e == KeyPress(Alt_L) {
                    asd = true;
                }
                if e == KeyRelease(Alt_L) {
                    asd = true;
                }
            }

        }
- Keysym = Vec<KeyCode>