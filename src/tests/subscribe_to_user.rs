// TODO: add tests for subscription to user

#[test]
fn subscribe_frequency_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 400;
		let frequency = 0;
		let number_of_instalments = Some(4);

		assert_noop!(
			PalletSubscription::subscribe_to_plan(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalments
			),
			Error::<TestRuntime>::InvalidFrequency
		);
	})
}

#[test]
fn subscribe_amount_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 0;
		let frequency = 5;
		let number_of_instalments = Some(4);

		assert_noop!(
			PalletSubscription::subscribe_to_plan(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalments
			),
			Error::<TestRuntime>::InvalidAmount
		);
	})
}

#[test]
fn subscribe_instalments_zero() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 400;
		let frequency = 10;
		let number_of_instalments = Some(0);

		assert_noop!(
			PalletSubscription::subscribe_to_plan(
				Origin::signed(ALICE()),
				BOB(),
				amount,
				frequency,
				number_of_instalments
			),
			Error::<TestRuntime>::InvalidNumberOfInstalment
		);
	})
}

#[test]
fn subscribe_number_of_installment_none() {
	ExternalityBuilder::default().build().execute_with(|| {
		let amount = 2000;
		let frequency = 4;
		let number_of_instalments = None;

		assert_ok!(PalletSubscription::subscribe_to_plan(
			Origin::signed(ALICE()),
			BOB(),
			amount,
			frequency,
			number_of_instalments
		));

		let expected_instalment = InstalmentData {
			frequency,
			amount,
			remaining_payments: number_of_instalments,
			beneficiary: BOB(),
			payer: ALICE(),
		};
		assert!(PalletSubscription::active_subscriptions(2).contains(&expected_instalment));

		let expected_event =
			Event::PalletSubscription(SubscriptionEvent::SubscriptionToPlan(expected_instalment));
		let received_event = &System::events()[0].event;

		assert_eq!(*received_event, expected_event);
	})
}
