use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn it_sets_clubs() {
    new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
		assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
		assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
    });
}

#[test]
fn it_assigns_to_club() {
    new_test_ext().execute_with(|| {
		let club_number: u8 = 0;
        let club_id: u32 = ClubsModule::number_to_id(club_number);
        let user: u64 = 1;
        
        assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
		assert_ok!(ClubsModule::assign_to_club(Origin::root(), user, club_number));
        
        let user_info = ClubsModule::users(user);
        // Succesfully added user to first (and only) club
        assert!(user_info == club_id);
    });
}

#[test]
fn it_removes_from_club() {
    new_test_ext().execute_with(|| {
		let club_number: u8 = 0;
        let user = 1;
        let club_id: u32 = ClubsModule::number_to_id(club_number);

        assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
		assert_ok!(ClubsModule::assign_to_club(Origin::root(), user, club_number));
        
        let user_info = ClubsModule::users(user);
        
        assert!(user_info == club_id);
		assert_ok!(ClubsModule::remove_from_club(Origin::root(), user, club_number));
        
        let user_info = ClubsModule::users(user);
        // Removed, clubs bitmask is empty
        assert!(user_info == 0);
    });
}

#[test]
fn it_multiple_clubs() {
    new_test_ext().execute_with(|| {
		// First bit at u32
        let club_number_0: u8 = 0;
		// Fourth bit at u32
        let club_number_3: u8 = 3;
        // Bitmask represents clubs, allowed for current user. 1 means allowed, 0 - not allowed.
        let expected_bitmask = 0b00000000000000000000000000001001_u32;
        let user = 1;

        // Add clubs
        for x in 1..5 { 
           assert_ok!(ClubsModule::add_club(Origin::root(), x));
        }
        // Assign user to some of them
        assert_ok!(ClubsModule::assign_to_club(Origin::root(), user, club_number_0));
        assert_ok!(ClubsModule::assign_to_club(Origin::root(), user, club_number_3));

        let user_info = ClubsModule::users(user);

        assert!(user_info == expected_bitmask);
		assert_ok!(ClubsModule::remove_from_club(Origin::root(), user, club_number_3));

        let user_info = ClubsModule::users(user);
        
        assert!(user_info == 1);
    });
}

#[test]
fn it_try_run_unrooted() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ClubsModule::add_club(Origin::signed(1), 1_u8), 
            BadOrigin
        );
        assert_noop!(
            ClubsModule::assign_to_club(Origin::signed(1), 1_u64, 1_u8), 
            BadOrigin
        );
        assert_noop!(
            ClubsModule::remove_from_club(Origin::signed(1), 1_u64, 1_u8), 
            BadOrigin
        );
    });
}

#[test]
fn it_try_add_one_club_twice() {
    new_test_ext().execute_with(|| {
		let club_number: u8 = 0;
        let user = 1;

        assert_ok!(ClubsModule::add_club(Origin::root(), 1_u8));
		assert_ok!(ClubsModule::assign_to_club(Origin::root(), user, club_number));
        assert_noop!(
            ClubsModule::assign_to_club(Origin::root(), user, club_number), 
            Error::<Test>::ClubAlreadySet);
    });
}

#[test]
fn it_try_assign_nonexistent_club() {
    new_test_ext().execute_with(|| {
		let club_number: u8 = 10;
        let user = 1;
        
        assert_noop!(
            ClubsModule::assign_to_club(Origin::root(), user, club_number), 
            Error::<Test>::InvalidClub);
    });
}

#[test]
fn it_try_overflowed_club() {
    new_test_ext().execute_with(|| {
        let club_number: u8 = 40;
        let user = 1;
        
        assert_noop!(
            ClubsModule::assign_to_club(Origin::root(), user, club_number), 
            Error::<Test>::IndexOutOfBounds
        );
    });
}