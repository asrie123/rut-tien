🚀 Soroban Simple Wallet Contract
📌 Title
Decentralized Asset Vault (DAV) - Hệ thống ví lưu ký phi tập trung trên Stellar Soroban.

📝 Description
Decentralized Asset Vault là một giải pháp Smart Contract cho phép người dùng quản lý tài sản số một cách minh bạch và an toàn tuyệt đối.

Mục đích: Tạo ra một "két sắt" kỹ thuật số nơi người dùng có thể nạp tiền (Token) vào và rút ra bất cứ lúc nào mà không cần sự can thiệp của bên thứ ba.

Tại sao làm ý tưởng này? Trong thế giới DeFi, việc tự quản lý tài sản (Self-custody) là yếu tố cốt lõi. Dự án này minh chứng cho khả năng lập trình tài chính (Programmable Money) trên mạng lưới Stellar, giúp người dùng làm quen với việc tương tác trực tiếp với các hợp đồng thông minh.

✨ Tính năng cụ thể
Nạp tiền (Deposit): Chuyển các loại Token (như XLM, USDC, v.v.) từ ví cá nhân vào Smart Contract. Hệ thống tự động ghi nhận số dư riêng biệt cho từng người dùng.

Rút tiền (Withdraw): Cho phép người dùng rút lại tài sản của mình từ Contract về ví cá nhân. Hệ thống kiểm soát chặt chẽ số dư khả dụng để ngăn chặn việc rút quá mức.

Xác thực bảo mật (Auth): Tích hợp cơ chế require_auth() của Soroban, đảm bảo chỉ chủ sở hữu ví mới có quyền thao tác trên tài sản của chính họ.

Truy vấn số dư (Query): * Kiểm tra số dư cá nhân hiện có trong Contract.

Kiểm tra tổng thanh khoản (Total Balance) mà Contract đang quản lý.

Hệ thống Sự kiện (Events): Tự động phát tín hiệu deposit và withdraw lên chuỗi khối để các ứng dụng Frontend có thể theo dõi lịch sử giao dịch thời gian thực.

📜 Contract
https://stellar.expert/explorer/testnet/contract/CCSACJN7IQY5P6JMM35WW7KD35QKARTLOK2QWUWFCNLFDLZ4Y2Y6UQN7

Ảnh chụp màn hình Contract:
<img width="1919" height="1079" alt="image" src="https://github.com/user-attachments/assets/46ac9612-3345-4f9a-a2b0-d5bafb53d9ed" />

🔭 Future Scopes
Hỗ trợ đa tài sản: Nâng cấp để quản lý danh mục nhiều loại token cùng lúc một cách tối ưu hơn.

Time-lock: Thêm tính năng khóa tiền trong một khoảng thời gian nhất định (thích hợp cho tiết kiệm dài hạn).

Phí quản lý (Fee System): Tích hợp cơ chế thu phí nhỏ để duy trì hoạt động và phát triển nền tảng.

Giao diện người dùng (UI/UX): Phát triển ứng dụng Web giúp người dùng phổ thông có thể nạp/rút tiền mà không cần sử dụng dòng lệnh (CLI).

👤 Profile
Nickname: [Tên/Nickname của bạn]

Kỹ năng: Rust, Soroban Smart Contracts, Stellar SDK, Blockchain Development.

Liên hệ: [Email hoặc link Social của bạn]

Cách tương tác với Contract qua CLI:
1. Nạp tiền:

Bash
stellar contract invoke --id CCSACJN7IQY5P6JMM35WW7KD35QKARTLOK2QWUWFCNLFDLZ4Y2Y6UQN7 --source-account <ALIAS> --network testnet -- deposit --user <ALIAS> --token_address <TOKEN_ID> --amount 100000000
2. Rút tiền:

Bash
stellar contract invoke --id CCSACJN7IQY5P6JMM35WW7KD35QKARTLOK2QWUWFCNLFDLZ4Y2Y6UQN7 --source-account <ALIAS> --network testnet -- withdraw --user <ALIAS> --token_address <TOKEN_ID> --amount 100000000
Dự án được thực hiện với mục tiêu nâng cao trải nghiệm lập trình trên hệ sinh thái Stellar.

Tôi có thể giúp gì thêm cho bạn?
Bạn có muốn mình bổ sung thêm phần Hướng dẫn cài đặt môi trường (Installation Guide) để người khác có thể chạy code của bạn ngay lập tức không?
