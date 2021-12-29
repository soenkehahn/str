FROM alpine:3.15

# install str
RUN apk add just yarn cargo bash make
ADD . /str/
WORKDIR /str
RUN just typescript-library-bundle
RUN just install

# create and run user test-suite
ADD tests/from-scratch/ /user-dir/
WORKDIR /user-dir
RUN yarn install
RUN yarn add /str/typescript-library/str.tgz

CMD ["str"]
