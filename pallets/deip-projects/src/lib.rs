#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode}, debug, ensure,
    decl_module, decl_storage, decl_event, decl_error, 
    StorageMap
};
use frame_system::{ self as system, ensure_signed };
use sp_std::vec::Vec;
use sp_std::vec;
use sp_runtime::{ RuntimeDebug };
use sp_core::{ H160 };


/// A maximum number of Domains. When domains reaches this number, no new domains can be added.
pub const MAX_DOMAINS: u32 = 100;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Project<Hash, H160, Domain, AccountId> {
    is_private: bool,
    external_id: H160,
    team: H160,
    description: Hash,
    domains: Vec<Domain>,
    members: Vec<AccountId>
    
}

type Domain = H160;
type ProjectOf<T> = Project<<T as system::Trait>::Hash, H160, Domain, <T as system::Trait>::AccountId>;

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
    pub enum Event<T> 
    where 
        AccountId = <T as frame_system::Trait>::AccountId,
        // Hash = <T as system::Trait>::Hash,
        Project = ProjectOf<T>
    {
        /// Event emitted when a project has been created. [BelongsTo, Project]
        ProjectCreated(AccountId, Project),
        /// Event emitted when a project is removed by the owner. [BelongsTo, Project]
        ProjectRemoved(AccountId, Project),

        /// Added a domain. [Creator, Domain]
		DomainAdded(AccountId, Domain),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        // ==== Projects ===
        
        /// The project does not exist, so it cannot be removed.
        NoSuchProject,
        /// The project is created by another account, so caller can't remove it.
        NotProjectOwner,
        /// Cannot add domain into the porject because this domain not exists
        DomainNotExists,

        // ==== Domains ===
        
        /// Cannot add another domain because the limit is already reached
        DomianLimitReached,
        /// Cannot add domain because this domain is already a exists
        DomainAlreadyExists,     
    }
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as DeipProjects {
        /// The storage item for our projects.
        /// It maps a projects to the user who created the them.
        Projects get(fn projects): map hasher(blake2_128_concat) T::AccountId => Vec<ProjectOf<T>>;

        // The set of all Domains.
        Domains get(fn domains): map hasher(blake2_128_concat) Domain => ();
        // The total number of domains stored in the map.
        // Because the map does not store its size, we must store it separately
        DomainCount: u32;
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Allow a user to create full project.
        #[weight = 10_000]
        fn create_project_full(origin, project: ProjectOf<T>) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;
            
            for domain in &project.domains {
                ensure!(Domains::contains_key(&domain), Error::<T>::DomainNotExists);
            }

            let mut projects = Projects::<T>::get(&account);

            // Modify Projects List via adding new Project
            projects.push(project.clone());

            // Store the projects related to account
            Projects::<T>::insert(&account, projects);

            // Emit an event that the project was created.
            Self::deposit_event(RawEvent::ProjectCreated(account, project));
        }
        
        /// Allow a user to create domains.
        #[weight = 10_000]
        fn add_domain(origin, doamin: Domain) {
            let account = ensure_signed(origin)?;
        
            let domain_count = DomainCount::get();
            ensure!(domain_count < MAX_DOMAINS, Error::<T>::DomianLimitReached);
        
            // We don't want to add duplicate doamins, so we check whether the potential new
            // domain is already present in the list. Because the domains is stored as a hash
            // map this check is constant time O(1)
            ensure!(!Domains::contains_key(&doamin), Error::<T>::DomainAlreadyExists);
        
            // Insert the new domin and emit the event
            Domains::insert(&doamin, ());
            DomainCount::put(domain_count + 1); // overflow check not necessary because of maximum
            Self::deposit_event(RawEvent::DomainAdded(account, doamin));
        }

    }
}
