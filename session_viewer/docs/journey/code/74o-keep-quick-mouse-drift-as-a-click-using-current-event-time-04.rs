pub fn drags(moved:f64,held:f64)->bool {moved>10. || (moved>4. && held>=200.)}

#[cfg(test)]
mod drift_tests {
    use super::*;
    #[test]
    fn a_quick_wobble_is_a_click() {
        assert!(!drags(5.,60.));assert!(!drags(9.,150.));assert!(drags(12.,40.));assert!(drags(5.,250.));assert!(!drags(3.,900.));
    }
    #[test]
    fn quick_and_held_mouse_events_keep_the_original_click_or_drag_owner() {
        let mut input=Gesture::default();assert!(input.press_at(1,0,[10.,20.],false,1000.));assert_eq!(input.move_at(1,[15.,20.],1060.),None);assert_eq!(input.release_at(1,[19.,20.],1150.),Some(Motion::Pick([19.,20.])));
        assert!(input.press_at(1,0,[10.,20.],false,2000.));assert_eq!(input.move_at(1,[15.,20.],2250.),Some(Motion::Probe {from:[10.,20.],to:[15.,20.]}));assert_eq!(input.release_at(1,[15.,20.],2260.),Some(Motion::Finish([15.,20.])));
        assert!(input.press_at(1,0,[0.,0.],false,3000.));assert_eq!(input.release_at(1,[12.,0.],3040.),Some(Motion::Probe {from:[0.,0.],to:[12.,0.]}));
    }
    #[test]
    fn invalid_timestamps_and_other_pointers_cannot_begin_a_drag() {
        let mut input=Gesture::default();assert!(!input.press_at(1,0,[0.,0.],false,f64::NAN));assert!(input.press_at(1,0,[0.,0.],false,1000.));assert_eq!(input.move_at(2,[20.,0.],1200.),None);assert_eq!(input.move_at(1,[20.,0.],900.),None);assert_eq!(input.move_at(1,[20.,0.],f64::NAN),None);assert_eq!(input.release_at(1,[3.,0.],1900.),Some(Motion::Pick([3.,0.])));
    }
}
