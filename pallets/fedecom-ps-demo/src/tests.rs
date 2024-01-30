use crate::{FlexibilitySellingData, FLEXIBILITY_SELLING_STATE_NOT_DECIDED, FLEXIBILITY_SELLING_STATE_CONFIRMED, FLEXIBILITY_SELLING_STATE_REJECTED, mock::*};
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
            1
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
            1
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
            1
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
            1
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
#[test]
fn try_to_sell_flexibility_confirmed() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let flexibility_market_identifier = 100;
        let flexibility_market_timestamp = 1234567890;
        let asset_identifier = 200;
        let sold_power = 10;
        let change_fct_w = 2;

        // Try to change the state of a not-existing market
        assert!(FedecomPSDemo::flexibility_purchase_decision(
            RuntimeOrigin::signed(buyer),
            seller,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            FLEXIBILITY_SELLING_STATE_NOT_DECIDED
        ).is_err());

        // Try to sell the flexibility
        assert_ok!(FedecomPSDemo::flexibility_selling(
            RuntimeOrigin::signed(seller),
            buyer,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            sold_power,
            change_fct_w)
        );

        // Check if the flexibility was correctly saved
        let flexibility_data = FlexibilitySellingData { sold_power, change_fct_w, state: FLEXIBILITY_SELLING_STATE_NOT_DECIDED };
        assert_eq!(FedecomPSDemo::get_flexibility_selling(
            seller,
            buyer,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier), flexibility_data
        );

        // Try to confirm the flexibility purchase
        assert_ok!(FedecomPSDemo::flexibility_purchase_decision(
            RuntimeOrigin::signed(buyer),
            seller,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            FLEXIBILITY_SELLING_STATE_CONFIRMED)
        );

        // Check if the flexibility was confirmed
        let flexibility_data = FlexibilitySellingData { sold_power, change_fct_w, state: FLEXIBILITY_SELLING_STATE_CONFIRMED };
        assert_eq!(FedecomPSDemo::get_flexibility_selling(
            seller,
            buyer,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier), flexibility_data
        );

        // Check the buyer's payment
        assert_eq!(FedecomPSDemo::get_payment(buyer, seller, flexibility_market_timestamp), sold_power * change_fct_w);

        // Try to change the state of the market, which has been already decided here above (in this case confirmed)
        assert!(FedecomPSDemo::flexibility_purchase_decision(
            RuntimeOrigin::signed(buyer),
            seller,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            FLEXIBILITY_SELLING_STATE_REJECTED
        ).is_err());
    })
}
#[test]
fn try_to_sell_flexibility_rejected() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let flexibility_market_identifier = 100;
        let flexibility_market_timestamp = 1234567890;
        let asset_identifier = 200;
        let sold_power = 10;
        let change_fct_w = 2;

        // Try to sell the flexibility
        assert_ok!(FedecomPSDemo::flexibility_selling(
            RuntimeOrigin::signed(seller),
            buyer,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            sold_power,
            change_fct_w)
        );

        // Try to reject the flexibility purchase
        assert_ok!(FedecomPSDemo::flexibility_purchase_decision(
            RuntimeOrigin::signed(buyer),
            seller,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier,
            FLEXIBILITY_SELLING_STATE_REJECTED)
        );

        // Check if the flexibility was rejected
        let flexibility_data = FlexibilitySellingData { sold_power, change_fct_w, state: FLEXIBILITY_SELLING_STATE_REJECTED };
        assert_eq!(FedecomPSDemo::get_flexibility_selling(
            seller,
            buyer,
            flexibility_market_identifier,
            flexibility_market_timestamp,
            asset_identifier), flexibility_data
        );

        // Check if the buyer's payment has not been performed
        assert_eq!(FedecomPSDemo::check_payment(buyer, seller, flexibility_market_timestamp), false);
    })
}


