#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode}, debug, ensure,
    decl_module, decl_storage, decl_event, decl_error, 
    StorageMap,
    dispatch::{ DispatchResult }
};
use frame_system::{ self as system, ensure_signed };
use sp_std::vec::Vec;
use sp_runtime::{ RuntimeDebug };
use sp_core::{ H160 };

/// A maximum number of Domains. When domains reaches this number, no new domains can be added.
pub const MAX_DOMAINS: u32 = 100;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
enum ResearchContentType {
    Announcemen,
    FinalResul,
    MilestoneArticl,
    MilestoneBoo,
    MilestoneChapte,
    MilestoneCod,
    MilestoneConferencePape,
    MilestoneCoverPag,
    MilestoneDat,
    MilestoneExperimentFindings,
    MilestoneMethod,
    MilestoneNegativeResults,
    MilestonePatent,
    MilestonePoster,
    MilestonePreprint,
    MilestonePresentation,
    MilestoneRawData,
    MilestoneResearchProposal,
    MilestoneTechnicalReport,
    MilestoneThesis,
}

// start_milestone_type = ResearchContentType::MilestoneArticle as isize,
// last_milestone_type = ResearchContentType::MilestoneThesis as isize,

impl Default for ResearchContentType {
    fn default() -> ResearchContentType { ResearchContentType::Announcemen}
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

pub type ProjectId = H160;
pub type Domain = H160;
pub type ProjectContentId = H160;
pub type ProjectOf<T> = Project<<T as system::Trait>::Hash, <T as system::Trait>::AccountId>;
pub type ProjectContentOf<T> = ProjectContent<<T as system::Trait>::Hash, <T as system::Trait>::AccountId>;

// TODO add is_finished calculated state field
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Project<Hash, AccountId> {
    is_private: bool,
    external_id: ProjectId,
    team_id: AccountId,
    description: Hash,
    domains: Vec<Domain>,
    members: Vec<AccountId>
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ProjectContent<Hash, AccountId> {
    external_id: ProjectContentId,
    project_external_id: ProjectId,
    team_id: AccountId,
    content_type: ResearchContentType,
    description: Hash,
    content: Hash,
    authors: Vec<AccountId>,
    references: Option<Vec<ProjectContentId>>
    
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event! {
    pub enum Event<T> 
    where 
        AccountId = <T as frame_system::Trait>::AccountId,
        Project = ProjectOf<T>,
        // Content = ProjectContentOf<T>
    {
        // ==== Projects ====

        /// Event emitted when a project has been created. [BelongsTo, Project]
        ProjectCreated(AccountId, Project),
        /// Event emitted when a project is removed by the owner. [BelongsTo, Project]
        ProjectRemoved(AccountId, Project),
        /// Event emitted when a project is removed by the owner. [BelongsTo, ProjectId]
        ProjectUpdated(AccountId, ProjectId),

        // ==== Project Content ====
       
        /// Event emitted when a project contnet has been created. [BelongsTo, ProjectContentId]
        ProjectContnetCreated(AccountId, ProjectContentId),

        /// Added a domain. [Creator, Domain]
		DomainAdded(AccountId, Domain),
    }
}

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        // ==== Projects ====
        
        /// The project does not exist.
        NoSuchProject,
        /// The project is created by another account, so caller can't remove it.
        NotProjectOwner,
        /// Cannot add domain into the porject because this domain not exists
        DomainNotExists,
        /// Cannot add a project because a project with this ID is already a exists
		ProjectAlreadyExists,

        // ==== Project Content ====
       
        /// Cannot add a project content because a project content with this ID is already a exists.
        ProjectContentAlreadyExists,
        /// Project does not belong to the team.
        ProjectNotBelongToTeam,
        /// The Reference does not exist.
        NoSuchReference, 
        /// Cannot add a project content because a project with this ID is already a finished
		ProjectAlreadyFinished,


        // ==== Domains ====
        
        /// Cannot add another domain because the limit is already reached
        DomianLimitReached,
        /// Cannot add domain because this domain is already a exists
        DomainAlreadyExists,
        
        
        // ==== General =====

        NoPermission,
    }
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as Deip {
        /// The storage item for our projects.
        ProjectMap get(fn project): map hasher(identity) ProjectId => ProjectOf<T>;
        /// This storage map of ProjectId and Creator
        Projects get(fn projects): Vec<(ProjectId, T::AccountId)>;

        ProjectContentMap get(fn project_content_entity): double_map hasher(identity) ProjectId, hasher(identity) ProjectContentId => ProjectContentOf<T>;
        ProjectsContent get(fn project_content_list): Vec<(ProjectContentId, ProjectId, T::AccountId)>;

        // The set of all Domains.
        Domains get(fn domains) config(): map hasher(blake2_128_concat) Domain => ();
        // The total number of domains stored in the map.
        // Because the map does not store its size, we must store it separately
        pub DomainCount config(domain_count): u32;
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
       
        /// Allow a user to create project.
        #[weight = 10_000]
        fn create_project(origin, project: ProjectOf<T>) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;
            
            for domain in &project.domains {
                ensure!(Domains::contains_key(&domain), Error::<T>::DomainNotExists);
            }

            let mut projects = Projects::<T>::get();

            // We don't want to add duplicate projects, so we check whether the potential new
			// project is already present in the list. Because the list is always ordered, we can
			// leverage the binary search which makes this check O(log n).
			match projects.binary_search_by_key(&project.external_id, |&(a,_)| a) {
				// If the search succeeds, the project is already a exists, so just return
				Ok(_) => return Err(Error::<T>::ProjectAlreadyExists.into()),
				// If the search fails, the project is not a exists and we learned the index where
				// they should be inserted
				Err(index) => {
					projects.insert(index, (project.external_id, project.team_id.clone()));
					Projects::<T>::put(projects);
				}
			};

            // Store the projects related to account
            ProjectMap::<T>::insert(project.external_id, project.clone());

            // Emit an event that the project was created.
            Self::deposit_event(RawEvent::ProjectCreated(account, project));
        }

        /// Allow a user to update project.
        #[weight = 10_000]
        fn update_project(origin, project_id: ProjectId, description: Option<T::Hash>, is_private: Option<bool>, members: Option<Vec<T::AccountId>>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let account = ensure_signed(origin)?;

            ProjectMap::<T>::mutate_exists(project_id, |maybe_project| -> DispatchResult {
                let project = maybe_project.as_mut().ok_or(Error::<T>::NoSuchProject)?;

                ensure!(project.team_id == account, Error::<T>::NoPermission);

                // TODO make sure that we don't lose first 2 bytes of the hash
                if let Some(value) = description  {
                    project.description = value;
                }

                if let Some(value) = is_private  {
                    project.is_private = value;
                }
                
                if let Some(value) = members  {
                    project.members = value;
                }

                Ok(())
            })?;

            // Emit an event that the project was updated.
            Self::deposit_event(RawEvent::ProjectUpdated(account, project_id));

            Ok(())
        }

        /// Allow a user to create project content.
        #[weight = 10_000]
        fn create_project_content(origin, content: ProjectContentOf<T>) {
            let account = ensure_signed(origin)?;

            let mut project_content = ProjectsContent::<T>::get();

            let index_to_insert_content = project_content.binary_search_by_key(&content.external_id, |&(a,_, _)| a)
                .err().ok_or(Error::<T>::ProjectContentAlreadyExists)?;

            let project = ProjectMap::<T>::get(content.project_external_id);

            ensure!(!project.external_id.is_zero(), Error::<T>::NoSuchProject);
            ensure!(project.team_id == content.team_id, Error::<T>::ProjectNotBelongToTeam);
            ensure!(!Self::is_project_finished(&project.external_id), Error::<T>::ProjectAlreadyFinished);


            if let Some(references) = &content.references {
                let is_all_references_exists = references
                    .iter()
                    .all(|&reference| project_content.binary_search_by_key(&reference,|&(id,_, _)| id).is_ok());

                ensure!(is_all_references_exists, Error::<T>::NoSuchReference);
            }

            project_content.insert(index_to_insert_content, (content.external_id, content.project_external_id,  content.team_id.clone()));
            ProjectsContent::<T>::put(project_content);

            // Store the content
            ProjectContentMap::<T>::insert(project.external_id, content.external_id, content.clone());

            // Emit an event that the content was created.
            Self::deposit_event(RawEvent::ProjectContnetCreated(account, content.external_id));
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

impl<T: Trait> Module<T> {
	fn is_project_finished(project_id: &ProjectId) -> bool {
		ProjectContentMap::<T>::iter_prefix_values(project_id)
            .any(|x| x.content_type == ResearchContentType::FinalResul)
	}
}
