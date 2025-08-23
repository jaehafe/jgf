# JGF (Jira Git Flow) 🚀

> Jira와 Git을 연동하는 워크플로우 자동화 CLI 도구

## 📌 문제 상황

개발팀에서 반복적으로 겪는 문제들:
- PM이 Jira 티켓 할당 → 개발자가 수동으로 티켓 확인
- 티켓 번호로 브랜치 생성 → 수동으로 Jira 상태 변경
- PR 생성 시 Jira 링크 복사/붙여넣기
- PR 머지 후 Jira 상태 수동 업데이트
- 로컬 브랜치 정리

**이 모든 과정이 시간이 많이 소요됩니다.**

## ✨ JGF가 해결하는 것

1. **자동 브랜치 생성**: Jira 티켓 번호로 자동 브랜치 생성
2. **자동 상태 동기화**: 작업 시작/PR/머지 시 Jira 상태 자동 업데이트
3. **PR 템플릿 자동화**: Jira 링크와 티켓 정보 자동 포함
4. **브랜치 자동 정리**: 머지된 브랜치 자동 감지 및 삭제

## 🔄 워크플로우

### 전체 플로우
```
1. PM/개발자 Jira 티켓 할당
    ↓
2. 개발자: jgf tickets (티켓 조회)
    ↓
3. 개발자: jgf start EM-XXX (브랜치 생성)
    → 자동: Git 브랜치 생성
    → 자동: Jira 상태 "In Progress"로 변경
    ↓
4. 개발자: 코딩 작업
    ↓
5. 개발자: jgf pr (PR 생성)
    → 자동: PR 제목에 티켓 번호 포함
    → 자동: PR 본문에 Jira 링크 추가
    ↓
6. 팀: 코드 리뷰 & 머지
    ↓
7. 개발자: jgf sync (동기화)
    → 자동: 머지된 브랜치 감지
    → 자동: Jira 상태 "Done"으로 변경
    → 자동: 로컬 브랜치 삭제
```

### 상태 변화
```
Jira 상태:  To Do → In Progress → Done
Git 브랜치:  없음 → EM-XXX 생성 → PR → 머지 → 삭제
```

## 🛠 설치 방법

### Prerequisites
- Rust 1.70+
- Git
- Jira 계정 및 API 토큰
- GitHub 계정 및 Personal Access Token

### 빌드 및 설치

```bash
# Clone
git clone https://github.com/yourusername/jgf.git
cd jgf

# 빌드
cargo build --release

# 시스템 전역 설치
sudo cp target/release/jgf /usr/local/bin/

# 또는 cargo install
cargo install --path .
```

## ⚙️ 초기 설정

각 프로젝트 루트에서 실행:

```bash
jgf init
```

`.env` 파일 생성 후 수정:
```env
# Jira 설정
JIRA_URL=https://your-company.atlassian.net
JIRA_PROJECT=EM
JIRA_USERNAME=your-email@company.com
JIRA_TOKEN=your-jira-api-token

# GitHub 설정
GITHUB_TOKEN=ghp_your_github_token
REPO_OWNER=YourOrg
REPO_NAME=your-repo

# Git 설정
DEFAULT_BRANCH=develop

# 프로젝트 설정
PROJECT_NAME=your-project
```

### API 토큰 발급 방법

**Jira API Token:**
1. [Atlassian Account Settings](https://id.atlassian.com/manage-profile/security/api-tokens) 접속
2. "Create API token" 클릭
3. 토큰 이름 입력 후 생성
4. 토큰 복사하여 `.env`에 저장

**GitHub Personal Access Token:**
1. GitHub Settings → Developer settings → Personal access tokens
2. "Generate new token (classic)" 클릭
3. 권한 선택: `repo` (전체)
4. 토큰 생성 및 복사

## 📚 사용법

### 1. 할당된 티켓 조회 및 작업 시작

```bash
# 할당된 모든 티켓 조회
jgf tickets

# 상태별 필터링
jgf tickets --status "In Progress"

# 최대 개수 제한
jgf tickets --limit 10
```

**인터랙티브 모드:**
- 티켓 목록에서 선택
- "브랜치 생성 및 In Progress로 변경" 선택
- 자동으로 브랜치 생성 및 Jira 상태 업데이트

### 2. 특정 티켓으로 작업 시작

```bash
jgf start EM-100
```

**자동 수행 작업:**
- ✅ develop 브랜치에서 최신 변경사항 pull
- ✅ `EM-100` 브랜치 생성 및 체크아웃
- ✅ Jira 티켓을 "In Progress"로 변경

### 3. PR 생성

```bash
jgf pr
```

**자동 수행 작업:**
- ✅ 현재 브랜치에서 develop으로 PR 생성
- ✅ PR 제목: `[EM-100] 티켓 제목`
- ✅ PR 본문에 Jira 링크 자동 포함
- ✅ PR이 이미 존재하면 링크 안내

### 4. 머지 후 동기화

```bash
jgf sync
```

**자동 수행 작업:**
- ✅ develop 브랜치로 전환 및 최신 pull
- ✅ 머지된 브랜치 감지
- ✅ 해당 Jira 티켓을 "Done"으로 변경
- ✅ 로컬 브랜치 삭제

## 🎯 실제 사용 시나리오

### 시나리오 1: 새 기능 개발

```bash
# 1. 할당된 티켓 확인
$ jgf tickets
🎫 [1] EM-120 사용자 프로필 기능 추가
   상태: To Do | 담당자: 김개발 | 우선순위: High

# 2. 작업 시작 (인터랙티브 선택 또는 직접 명령)
$ jgf start EM-120
🚀 티켓 EM-120 작업을 시작합니다
🌿 브랜치 'EM-120'가 생성되고 체크아웃되었습니다
✅ 티켓 상태가 'In Progress'로 변경되었습니다

# 3. 코딩 작업...

# 4. PR 생성
$ jgf pr
🚀 브랜치 'EM-120'에서 'develop'으로 PR 생성
✅ PR이 성공적으로 생성되었습니다! #123
💡 PR 링크: https://github.com/YourOrg/your-repo/pull/123

# 5. 리뷰 & 머지 후
$ jgf sync
🔄 머지된 브랜치 동기화 시작
✅ 티켓 EM-120 상태가 'Done'으로 변경되었습니다
✅ 브랜치 'EM-120'가 삭제되었습니다
✨ 브랜치 동기화 완료!
```

### 시나리오 2: 여러 티켓 동시 작업

```bash
# 여러 브랜치에서 작업 후 한번에 정리
$ jgf sync
🔄 3개의 티켓 브랜치를 발견했습니다

🌿 브랜치 'EM-118' 확인 중...
✅ 브랜치 'EM-118'가 머지되었습니다
> 티켓 EM-118를 'Done' 상태로 변경하시겠습니까? Yes
> 로컬 브랜치 'EM-118'를 삭제하시겠습니까? Yes

🌿 브랜치 'EM-119' 확인 중...
💡 브랜치 'EM-119'는 아직 머지되지 않았습니다

🌿 브랜치 'EM-120' 확인 중...
✅ 브랜치 'EM-120'가 머지되었습니다
...
```

## 🏢 회사별 커스터마이징

### Jira 상태 매핑
기본적으로 다음 상태를 지원합니다:
- `To Do` / `해야 할 일`
- `In Progress` / `진행 중`
- `Done` / `완료`

회사에 "In Review" 상태가 없는 경우, PR 생성 시 상태를 변경하지 않고 머지 후 Done으로만 변경합니다.

### 브랜치 네이밍
- 기본: `{JIRA_TICKET_NUMBER}` (예: `EM-100`)
- 수정 필요시 `src/config.rs`의 `format_branch_name()` 함수 수정

## 🔧 문제 해결

### SSH 인증 오류
```bash
# SSH 에이전트 확인
ssh-add -l

# SSH 키 추가
ssh-add ~/.ssh/id_ed25519
```

### Jira API 오류
- API 토큰이 올바른지 확인
- Jira URL이 `https://`로 시작하는지 확인
- 프로젝트 키(예: EM)가 정확한지 확인

### GitHub API 오류
- Personal Access Token 권한 확인 (repo 권한 필요)
- Repository owner와 name이 정확한지 확인
