FROM ubuntu:21.10

RUN apt-get update && apt-get install --yes curl yarnpkg make
RUN ln -s /usr/bin/yarnpkg /usr/local/bin/yarn
ADD tests/from-scratch/ /user-dir/
WORKDIR /user-dir
RUN yarn install
ADD . /str/
RUN yarn add --dev /str/typescript-library/str.tgz

CMD ["str"]
