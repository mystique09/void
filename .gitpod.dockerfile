FROM gitpod/workspace-postgresql

USER gitpod
ENV PATH="$HOME/.cargo/bin:$PATH"
RUN cargo install sqlx-cli

# TODO!
# Create new database user
# RUN createuser --host localhost -d -l -s mystique09
# RUN createdb --host=localhost class-manager

# Server config environment variables
ENV TOKEN=
ENV DATABASE_URL=postgres://gitpod@localhost/postgres?sslmode=disable

USER root