$VERSION="0.0.4"
cargo sqlx prepare --workspace --all -- --all-targets --all-features
git add .sqlx
git commit -m "New sqlx prepare statements"
docker build --tag "voxfila:$VERSION" .
docker tag "voxfila:$VERSION" shansanear/voxfila
docker push shansanear/voxfila