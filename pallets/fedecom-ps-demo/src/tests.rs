use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn check_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), true);
    });
}

#[test]
fn add_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);
    });
}

#[test]
fn remove_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), false);

        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), true);

        assert_ok!(FedecomPSDemo::remove_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ));

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), false);
    });
}

#[test]
fn modify_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;
        let value_to_set = 200;

        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        assert_ok!(FedecomPSDemo::modify_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_set
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_set);
    });
}

#[test]
fn insert_element_two_times() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        // Transaction returning an error
        assert!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert+1
        ).is_err());

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);
    });
}

#[test]
fn remove_payment_two_times() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), false);

        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), true);

        assert_ok!(FedecomPSDemo::remove_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ));

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), false);

        // Transaction returning an error because the element is not stored in the ledger
        assert!(FedecomPSDemo::remove_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ).is_err());

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), false);
    });
}

#[test]
fn add_confirmation() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction: add payment
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        // Transaction: add confirmation
        assert_ok!(FedecomPSDemo::add_confirmation(
            RuntimeOrigin::signed(receiver),
            sender,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_confirmation(sender, receiver, timestamp), value_to_insert);
    });
}

#[test]
fn remove_confirmation() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction: add payment
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        // Transaction: add confirmation
        assert_ok!(FedecomPSDemo::add_confirmation(
            RuntimeOrigin::signed(receiver),
            sender,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), true);

        // Transaction: remove confirmation
        assert_ok!(FedecomPSDemo::remove_confirmation(
            RuntimeOrigin::signed(receiver),
            sender,
            timestamp
        ));

        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), false);
    });
}

#[test]
fn try_to_remove_confirmed_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction: add payment
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        // Transaction: add confirmation
        assert_ok!(FedecomPSDemo::add_confirmation(
            RuntimeOrigin::signed(receiver),
            sender,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), true);

        // Transaction: try to remove the payment
        assert!(FedecomPSDemo::remove_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp
        ).is_err());

        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), true);
        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), true);
    });
}
#[test]
fn try_to_modify_confirmed_payment() {
    new_test_ext().execute_with(|| {
        let sender = 1;
        let receiver = 2;
        let timestamp = 1234567890;
        let value_to_insert = 100;

        // Transaction: add payment
        assert_ok!(FedecomPSDemo::add_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);

        // Transaction: add confirmation
        assert_ok!(FedecomPSDemo::add_confirmation(
            RuntimeOrigin::signed(receiver),
            sender,
            timestamp,
            value_to_insert
        ));

        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), true);

        // Transaction: try to modify the payment
        assert!(FedecomPSDemo::modify_payment(
            RuntimeOrigin::signed(sender),
            receiver,
            timestamp,
            value_to_insert+1
        ).is_err());

        assert_eq!(FedecomPSDemo::get_payment(sender, receiver, timestamp), value_to_insert);
        assert_eq!(FedecomPSDemo::check_payment(sender, receiver, timestamp), true);
        assert_eq!(FedecomPSDemo::check_confirmation(sender, receiver, timestamp), true);
    });
}


