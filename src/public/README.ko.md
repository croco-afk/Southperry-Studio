### MapleLens

메이플스토리 Wz 파일에서 스킬 애니메이션을 미리 보고 추출할 수 있는 도구입니다. 주로 "Southperry" 플레이어들이 커스텀 스킬을 제작할 때 필요한 리소스를 확보하기 위해 사용됩니다.

커스텀 스킬 주소: http://southperry.site/custom-skills

**핵심 기능**

- **WZ 파싱 및 검색:** `Base.wz` 파일의 드래그 앤 드롭 로딩을 지원합니다. 직업, ID 또는 이름으로 스킬을 빠르게 검색할 수 있습니다.
- **미리보기:** 스킬 애니메이션과 효과음을 완벽하게 복원하며, 기준점(Anchor) 표시를 지원합니다.
- **원클릭 리소스 추출:** 스킬 아이콘, 오디오 및 WebP 애니메이션을 일괄 내보낼 수 있습니다. 저장 경로를 직접 지정할 수 있습니다.
- **개발자 맞춤형 내보내기:** 내보낸 애니메이션에는 좌표(`origin`)와 지연 시간(`delay`)이 포함된 JSON 파일이 함께 생성됩니다. "최소 크기 자르기(Minimal Crop)" 또는 "캔버스 크기(Canvas Size)" 모드를 지원합니다.
- **다국어 지원:** 중국어, 영어, 한국어 인터페이스 전환을 완벽하게 지원합니다.

**WZ 리소스 획득**

모든 버전의 메이플스토리 클라이언트를 설치하고, 설치 폴더 내의 `Base.wz` 파일을 MapleLens 프로그램으로 드래그 앤 드롭하면 됩니다.
다른 버전의 메이플스토리 클라이언트 다운로드에 대해서는 다음 게시물을 참고할 수 있습니다:
https://forum.ragezone.com/threads/maplestory-client-localhost-archive.1101897/
https://archive.org/details/twms-maplestory

**기술 스택 (Tech Stack)**
- Frontend: Vue 3, Vite
- Backend: Rust (Tauri v2)

**감사의 말 및 출처 (Acknowledgement & Source):**

이 프로젝트의 **핵심 데이터 처리 백엔드**(Wz 파일 파싱, 노드 조회, 이미지/오디오 추출 등)는 **[MapleSalon2](https://github.com/spd789562/MapleSalon2)의 Rust 백엔드 코드를 직접 사용하였습니다.**

훌륭한 작업을 해주신 원작자 Leo Lin 님께 깊은 감사를 드립니다.

**원본 프로젝트 링크:** https://github.com/spd789562/MapleSalon2