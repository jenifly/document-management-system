-- 创建文档权限表
-- 注意：permission 使用 TEXT 类型而不是 ENUM，以便在应用层进行类型转换
CREATE TABLE document_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission TEXT NOT NULL CHECK (permission IN ('read', 'write', 'delete', 'share', 'admin')),
    granted_by UUID NOT NULL REFERENCES users(id),
    granted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    UNIQUE(document_id, user_id, permission)
);

-- 创建组表
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建组成员表
CREATE TABLE group_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(group_id, user_id)
);

-- 创建组权限表
CREATE TABLE group_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    group_id UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    permission TEXT NOT NULL CHECK (permission IN ('read', 'write', 'delete', 'share', 'admin')),
    granted_by UUID NOT NULL REFERENCES users(id),
    granted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    UNIQUE(document_id, group_id, permission)
);

-- 创建分享链接表
CREATE TABLE share_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    token VARCHAR(100) NOT NULL UNIQUE,
    created_by UUID NOT NULL REFERENCES users(id),
    permission TEXT NOT NULL DEFAULT 'read' CHECK (permission IN ('read', 'write', 'delete', 'share', 'admin')),
    password_hash VARCHAR(255),
    max_access_count INTEGER,
    access_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建访问日志表
CREATE TABLE access_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_document_permissions_document ON document_permissions(document_id);
CREATE INDEX idx_document_permissions_user ON document_permissions(user_id);
CREATE INDEX idx_group_members_group ON group_members(group_id);
CREATE INDEX idx_group_members_user ON group_members(user_id);
CREATE INDEX idx_group_permissions_document ON group_permissions(document_id);
CREATE INDEX idx_group_permissions_group ON group_permissions(group_id);
CREATE INDEX idx_share_links_token ON share_links(token);
CREATE INDEX idx_access_logs_document ON access_logs(document_id);
CREATE INDEX idx_access_logs_user ON access_logs(user_id);
CREATE INDEX idx_access_logs_created_at ON access_logs(created_at);

