FROM ubuntu:21.10

RUN apt-get update && apt-get install --yes curl yarnpkg make
RUN ln -s /usr/bin/yarnpkg /usr/local/bin/yarn
ADD tests/from-scratch/ /user-dir/
WORKDIR /user-dir
RUN yarn install
COPY typescript-library/str.tgz /str.tgz
RUN yarn add --dev /str.tgz

CMD ["str"]
