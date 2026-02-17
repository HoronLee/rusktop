#!/bin/bash
# 快速创建新 SeaORM migration
#
# 用法: ./scripts/new-migration.sh add_email_to_users

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <migration_description>"
    echo ""
    echo "Examples:"
    echo "  $0 add_email_to_users"
    echo "  $0 create_posts_table"
    exit 1
fi

DESCRIPTION=$1
DATE=$(date +%Y%m%d)
MIGRATION_DIR="crates/rusktop-core/src/migration"

COUNTER=$(ls -1 ${MIGRATION_DIR}/m${DATE}_*.rs 2>/dev/null | wc -l | tr -d ' ')
COUNTER=$(printf "%06d" $((COUNTER + 1)))

NAME="m${DATE}_${COUNTER}_${DESCRIPTION}"
FILE="${MIGRATION_DIR}/${NAME}.rs"

cat > "$FILE" << 'EOF'
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: 实现数据库变更
        // 示例：添加列
        // manager.alter_table(
        //     Table::alter()
        //         .table(User::Table)
        //         .add_column(ColumnDef::new(User::Email).string())
        //         .to_owned()
        // ).await
        
        todo!("实现 up migration")
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: 实现回滚逻辑
        // manager.alter_table(
        //     Table::alter()
        //         .table(User::Table)
        //         .drop_column(User::Email)
        //         .to_owned()
        // ).await
        
        todo!("实现 down migration")
    }
}

// TODO: 定义表和列的标识符
// #[derive(DeriveIden)]
// enum User {
//     Table,
//     Email,
// }
EOF

echo "mod ${NAME};" >> ${MIGRATION_DIR}/mod.rs

MOD_FILE="${MIGRATION_DIR}/mod.rs"
if grep -q "vec!\[Box::new" "$MOD_FILE"; then
    sed -i '' "s/vec!\[/vec![Box::new(${NAME}::Migration), /" "$MOD_FILE"
else
    echo "⚠️  Warning: Could not auto-update migrations list. Please manually add:"
    echo "   Box::new(${NAME}::Migration)"
fi

echo ""
echo "✅ Created migration: $FILE"
echo "✅ Updated $MOD_FILE"
echo ""
echo "📝 Next steps:"
echo "   1. Edit $FILE"
echo "   2. Replace TODO with actual migration logic"
echo "   3. Run: cargo run -- web"
echo ""
