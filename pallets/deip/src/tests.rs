use crate::*;
use crate::{mock::*};
use sp_core::H256;
use frame_support::{assert_ok, assert_noop};

#[test]
fn add_domain() {
	new_test_ext().execute_with(|| {
		let domain = Domain::random();
		// Dispatch a signed add domian extrinsic.
		assert_ok!(Deip::add_domain(Origin::signed(1), domain.clone()));
		
		// Read pallet storage and assert an expected result.
		assert_eq!(Deip::domain_count(), 1);
		assert!(
			<Domains>::contains_key(domain),
			"Domains did not contain domain, value was `{}`",
            domain
		);
	});
}

#[test]
fn cant_add_duplicate_domain() {
	new_test_ext().execute_with(|| {
		let domain = Domain::random();
		
		assert_ok!(Deip::add_domain(Origin::signed(1), domain.clone()));

		assert_noop!(
			Deip::add_domain(Origin::signed(1), domain.clone()),
			Error::<Test>::DomainAlreadyExists
		);
	})
}

#[test]
fn add_project() {
	new_test_ext().execute_with(|| {
		let domain = Domain::random();
		let account_id = 2;
		let project_id = ProjectId::random();
		
		assert_ok!(Deip::add_domain(Origin::signed(1), domain.clone()));

		let project = Project {
			is_private: false,
			external_id: project_id,
			team_id: account_id,
			description: H256::random(),
			domains: vec![domain],
			members: vec![account_id],
		};
		
		assert_ok!(Deip::create_project(Origin::signed(1), project.clone()));

		let projects = Projects::<Test>::get();
		let project_stored = ProjectMap::<Test>::get(project_id);

		assert!(
			<ProjectMap<Test>>::contains_key(project_id),
			"Domains did not contain domain, value was `{}`",
            project_id
		);

		assert_eq!(project, project_stored);

		assert!(
			projects.binary_search_by_key(&project_id, |&(external_id, ..)| external_id).is_ok(),
			"Projects did not contain project, value was `{}`",
            project_id
		);
		assert!(
			projects.binary_search_by_key(&project_id, |&(external_id, ..)| external_id).is_ok(),
			"Projects did not contain project, value was `{}`",
            project_id
		);

	})
}

#[test]
fn cant_add_project_with_non_exixsted_domain() {
	new_test_ext().execute_with(|| {
		let domain = Domain::random();
		let account_id = 2;
		
		let project = Project {
			is_private: false,
			external_id: ProjectId::random(),
			team_id: account_id,
			description: H256::random(),
			domains: vec![domain],
			members: vec![account_id],
		};
		
		assert_noop!(
			Deip::create_project(Origin::signed(1), project.clone()),
			Error::<Test>::DomainNotExists
		);
	})
}

