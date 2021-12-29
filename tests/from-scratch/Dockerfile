FROM alpine:3.15

# install str
RUN apk add just yarn cargo bash
ADD . /str/
WORKDIR /str
RUN just typescript-library-bundle
RUN just install

# create and run user test-suite
ADD tests/from-scratch/ /user-dir/
WORKDIR /user-dir
RUN yarn install
RUN yarn add /str/typescript-library/str-v0.0.0.tgz

CMD ["str"]
