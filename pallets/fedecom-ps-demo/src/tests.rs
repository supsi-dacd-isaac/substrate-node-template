use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn test_exist_query() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), true);
    });
}

#[test]
fn test_insert_element() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_element(sender, receiver, timestamp), value_to_insert);
    });
}

#[test]
fn test_remove_element() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), false);

        assert_ok!(FedecomPSDemo::add_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), true);

        assert_ok!(FedecomPSDemo::remove_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ));

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), false);
    });
}

#[test]
fn test_set_element() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;
        let value_to_set = 200;

        assert_ok!(FedecomPSDemo::add_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_element(sender, receiver, timestamp), value_to_insert);

        assert_ok!(FedecomPSDemo::set_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_set
        ));

        assert_eq!(FedecomPSDemo::get_element(sender, receiver, timestamp), value_to_set);
    });
}

#[test]
fn test_insert_element_two_times() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_element_with_error(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_element(sender, receiver, timestamp), value_to_insert);

        // Transaction returning an error
        assert!(FedecomPSDemo::add_element_with_error(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert+1
        ).is_err());

        assert_eq!(FedecomPSDemo::get_element(sender, receiver, timestamp), value_to_insert);
    });
}

#[test]
fn test_remove_element_two_times() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), false);

        assert_ok!(FedecomPSDemo::add_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), true);

        assert_ok!(FedecomPSDemo::remove_element(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ));

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), false);

        // Transaction returning an error because the element is not stored in the ledger
        assert!(FedecomPSDemo::remove_element_with_error(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ).is_err());

        assert_eq!(FedecomPSDemo::exist(sender, receiver, timestamp), false);
    });
}
