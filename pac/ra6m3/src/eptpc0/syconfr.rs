#[doc = "Register `SYCONFR` reader"]
pub struct R(crate::R<SYCONFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYCONFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYCONFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYCONFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYCONFR` writer"]
pub struct W(crate::W<SYCONFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYCONFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYCONFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYCONFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCYC` reader - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles)."]
pub type TCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCYC` writer - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles)."]
pub type TCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYCONFR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SBDIS` reader - Sync Message Transmission Bandwidth Securing Disable"]
pub type SBDIS_R = crate::BitReader<SBDIS_A>;
#[doc = "Sync Message Transmission Bandwidth Securing Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDIS_A {
    #[doc = "0: Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority)."]
    _0 = 0,
    #[doc = "1: Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority)."]
    _1 = 1,
}
impl From<SBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SBDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBDIS_A {
        match self.bits {
            false => SBDIS_A::_0,
            true => SBDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDIS_A::_1
    }
}
#[doc = "Field `SBDIS` writer - Sync Message Transmission Bandwidth Securing Disable"]
pub type SBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYCONFR_SPEC, SBDIS_A, O>;
impl<'a, const O: u8> SBDIS_W<'a, O> {
    #[doc = "Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBDIS_A::_0)
    }
    #[doc = "Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBDIS_A::_1)
    }
}
#[doc = "Field `FILDIS` reader - Receive Message domainNumber Filter Disable"]
pub type FILDIS_R = crate::BitReader<FILDIS_A>;
#[doc = "Receive Message domainNumber Filter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILDIS_A {
    #[doc = "0: Filtering conditions for the reception of PTP messages include comparison with the domainNumber field."]
    _0 = 0,
    #[doc = "1: Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field."]
    _1 = 1,
}
impl From<FILDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FILDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FILDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILDIS_A {
        match self.bits {
            false => FILDIS_A::_0,
            true => FILDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILDIS_A::_1
    }
}
#[doc = "Field `FILDIS` writer - Receive Message domainNumber Filter Disable"]
pub type FILDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYCONFR_SPEC, FILDIS_A, O>;
impl<'a, const O: u8> FILDIS_W<'a, O> {
    #[doc = "Filtering conditions for the reception of PTP messages include comparison with the domainNumber field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILDIS_A::_0)
    }
    #[doc = "Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILDIS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles)."]
    #[inline(always)]
    pub fn tcyc(&self) -> TCYC_R {
        TCYC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Sync Message Transmission Bandwidth Securing Disable"]
    #[inline(always)]
    pub fn sbdis(&self) -> SBDIS_R {
        SBDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Message domainNumber Filter Disable"]
    #[inline(always)]
    pub fn fildis(&self) -> FILDIS_R {
        FILDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles)."]
    #[inline(always)]
    #[must_use]
    pub fn tcyc(&mut self) -> TCYC_W<0> {
        TCYC_W::new(self)
    }
    #[doc = "Bit 12 - Sync Message Transmission Bandwidth Securing Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sbdis(&mut self) -> SBDIS_W<12> {
        SBDIS_W::new(self)
    }
    #[doc = "Bit 16 - Receive Message domainNumber Filter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fildis(&mut self) -> FILDIS_W<16> {
        FILDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP Operation Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syconfr](index.html) module"]
pub struct SYCONFR_SPEC;
impl crate::RegisterSpec for SYCONFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syconfr::R](R) reader structure"]
impl crate::Readable for SYCONFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syconfr::W](W) writer structure"]
impl crate::Writable for SYCONFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYCONFR to value 0"]
impl crate::Resettable for SYCONFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
