use crate::{AppResult, AppErrorType};
use crate::error::AppErrorExt;
use git2::{Repository, BranchType};

pub struct GitOps {
    repo: Repository,
}

impl GitOps {
    pub fn open() -> AppResult<Self> {
        let repo = Repository::open(".")
            .with_app_type(AppErrorType::GitError("Git 저장소를 찾을 수 없습니다".to_string()))?;
        
        Ok(GitOps { repo })
    }
    
    pub fn get_current_branch(&self) -> AppResult<String> {
        let head = self.repo.head()
            .with_app_type(AppErrorType::GitNoCurrentBranch)?;
        
        if let Some(branch_name) = head.shorthand() {
            Ok(branch_name.to_string())
        } else {
            Err(AppErrorType::GitNoCurrentBranch.into())
        }
    }
    
    pub fn is_clean_working_directory(&self) -> AppResult<bool> {
        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(true)
            .include_ignored(false)
            .include_unreadable(false);
        
        let statuses = self.repo.statuses(Some(&mut opts))
            .with_app_type(AppErrorType::GitError("상태 확인 실패".to_string()))?;
        
        Ok(statuses.is_empty())
    }
    
    pub fn branch_exists(&self, branch_name: &str) -> AppResult<bool> {
        match self.repo.find_branch(branch_name, BranchType::Local) {
            Ok(_) => Ok(true),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
            Err(e) => Err(AppErrorType::GitError(format!("브랜치 확인 실패: {}", e)).into()),
        }
    }
    
    pub fn create_and_checkout_branch(&self, branch_name: &str, base_branch: &str) -> AppResult<()> {
        if self.branch_exists(branch_name)? {
            return Err(AppErrorType::GitBranchExists.into());
        }
        
        if !self.is_clean_working_directory()? {
            return Err(AppErrorType::GitUncommittedChanges.into());
        }
        
        let base_branch_ref = self.repo.find_branch(base_branch, BranchType::Local)
            .or_else(|_| self.repo.find_branch(&format!("origin/{}", base_branch), BranchType::Remote))
            .with_app_type(AppErrorType::GitError(format!("기본 브랜치 '{}' 를 찾을 수 없습니다", base_branch)))?;
        
        let target_commit = base_branch_ref.get().peel_to_commit()
            .with_app_type(AppErrorType::GitError("커밋을 찾을 수 없습니다".to_string()))?;
        
        let new_branch = self.repo.branch(branch_name, &target_commit, false)
            .with_app_type(AppErrorType::GitError(format!("브랜치 '{}' 생성 실패", branch_name)))?;
        
        let branch_ref = new_branch.get();
        self.repo.set_head(branch_ref.name().unwrap())
            .with_app_type(AppErrorType::GitError("브랜치 체크아웃 실패".to_string()))?;
        
        self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .with_app_type(AppErrorType::GitError("워킹 디렉토리 업데이트 실패".to_string()))?;
        
        Ok(())
    }
    
    pub fn checkout_branch(&self, branch_name: &str) -> AppResult<()> {
        if !self.branch_exists(branch_name)? {
            return Err(AppErrorType::GitError(format!("브랜치 '{}'가 존재하지 않습니다", branch_name)).into());
        }
        
        if !self.is_clean_working_directory()? {
            return Err(AppErrorType::GitUncommittedChanges.into());
        }
        
        let branch = self.repo.find_branch(branch_name, BranchType::Local)
            .with_app_type(AppErrorType::GitError(format!("브랜치 '{}' 를 찾을 수 없습니다", branch_name)))?;
        
        let branch_ref = branch.get();
        self.repo.set_head(branch_ref.name().unwrap())
            .with_app_type(AppErrorType::GitError("브랜치 체크아웃 실패".to_string()))?;
        
        self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
            .with_app_type(AppErrorType::GitError("워킹 디렉토리 업데이트 실패".to_string()))?;
        
        Ok(())
    }
    
    pub fn pull_latest(&self, branch_name: &str) -> AppResult<()> {
        let current_branch = self.get_current_branch()?;
        if current_branch != branch_name {
            self.checkout_branch(branch_name)?;
        }
        
        use std::process::Command;
        
        let output = Command::new("git")
            .args(&["pull", "origin", branch_name])
            .output()
            .with_app_type(AppErrorType::GitError("Git pull 명령 실행 실패".to_string()))?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppErrorType::GitError(format!("Git pull 실패: {}", error_msg)).into());
        }
        
        Ok(())
    }
    
    pub fn get_remote_url(&self) -> AppResult<String> {
        let remote = self.repo.find_remote("origin")
            .with_app_type(AppErrorType::GitError("origin 리모트를 찾을 수 없습니다".to_string()))?;
        
        if let Some(url) = remote.url() {
            Ok(url.to_string())
        } else {
            Err(AppErrorType::GitError("리모트 URL을 가져올 수 없습니다".to_string()).into())
        }
    }
    
    pub fn list_branches(&self) -> AppResult<Vec<String>> {
        let branches = self.repo.branches(Some(BranchType::Local))
            .with_app_type(AppErrorType::GitError("브랜치 목록 가져오기 실패".to_string()))?;
        
        let mut branch_names = Vec::new();
        for branch in branches {
            let (branch, _) = branch
                .with_app_type(AppErrorType::GitError("브랜치 정보 가져오기 실패".to_string()))?;
            
            if let Some(name) = branch.name()
                .with_app_type(AppErrorType::GitError("브랜치 이름 가져오기 실패".to_string()))? {
                branch_names.push(name.to_string());
            }
        }
        
        Ok(branch_names)
    }
}