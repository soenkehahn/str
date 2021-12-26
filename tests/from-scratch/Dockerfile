FROM alpine:3.15

# install str
RUN apk add just yarn cargo
ADD . /str/
WORKDIR /str
RUN just install

# create and run user test-suite
ADD tests/from-scratch/ /user-dir/
WORKDIR /user-dir
RUN yarn install
RUN yarn add link:/str/typescript-library

CMD ["str"]
