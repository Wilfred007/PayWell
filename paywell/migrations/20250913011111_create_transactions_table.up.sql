-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS transactions (
    id CHAR(36) PRIMARY KEY NOT NULL,
    user_id CHAR(36) NOT NULL,
    amount DECIMAL(12,2) NOT NULL,
    bank_account_number VARCHAR(20) NOT NULL,
    bank_code VARCHAR(10) NOT NULL,
    reference VARCHAR(100) UNIQUE NOT NULL,
    status ENUM('Pending', 'Processing', 'Completed', 'Failed') DEFAULT 'Pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

