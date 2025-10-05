<script setup lang="ts">
import NetworkNameIDComponent from './overview/NetworkNameID.vue'
import NetworkCreationTimeComponent from './overview/NetworkCreateTime.vue'
import NetworkTypeComponent from './overview/NetworkType.vue'
import NetworkMTUComponent from './overview/NetworkOther.vue'
import NetworkIPAssignmentPoolsComponent from './overview/NetworkIPAssignmentPools.vue'
import NetworkIPv4AssignmentComponent from './overview/NetworkIPv4Assignment.vue'
import NetworkIPv6AssignmentComponent from './overview/NetworkIPv6Assignment.vue'
import NetworkDNSComponent from './overview/NetworkDNS.vue'
import NetworkRoutesComponent from './overview/NetworkRoutes.vue'
import NetworkRulesComponent from './overview/NetworkRules.vue'
</script>

<template>
  <div class="p-4 sm:grid-cols-1 grid lg:grid-cols-4 gap-4 w-full">
    <NetworkNameIDComponent />
    <NetworkCreationTimeComponent />
    <NetworkTypeComponent />
    <NetworkMTUComponent />
    <NetworkIPAssignmentPoolsComponent />
    <NetworkIPv4AssignmentComponent />
    <NetworkIPv6AssignmentComponent />
    <NetworkRoutesComponent />
    <NetworkDNSComponent />
    <NetworkRulesComponent />
  </div>
</template>
