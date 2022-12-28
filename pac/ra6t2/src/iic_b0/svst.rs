#[doc = "Register `SVST` reader"]
pub struct R(crate::R<SVST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVST` writer"]
pub struct W(crate::W<SVST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVST_SPEC>;
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
impl From<crate::W<SVST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCAF` reader - General Call Address Detection Flag"]
pub type GCAF_R = crate::BitReader<GCAF_A>;
#[doc = "General Call Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAF_A {
    #[doc = "0: General call address does not detect."]
    _0 = 0,
    #[doc = "1: General call address detects."]
    _1 = 1,
}
impl From<GCAF_A> for bool {
    #[inline(always)]
    fn from(variant: GCAF_A) -> Self {
        variant as u8 != 0
    }
}
impl GCAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAF_A {
        match self.bits {
            false => GCAF_A::_0,
            true => GCAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAF_A::_1
    }
}
#[doc = "Field `GCAF` writer - General Call Address Detection Flag"]
pub type GCAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, GCAF_A, O>;
impl<'a, const O: u8> GCAF_W<'a, O> {
    #[doc = "General call address does not detect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAF_A::_0)
    }
    #[doc = "General call address detects."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAF_A::_1)
    }
}
#[doc = "Field `HSMCF` reader - Hs-mode Master Code Detection Flag"]
pub type HSMCF_R = crate::BitReader<HSMCF_A>;
#[doc = "Hs-mode Master Code Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSMCF_A {
    #[doc = "0: Hs-mode Master Code does not detect."]
    _0 = 0,
    #[doc = "1: Hs-mode Master Code detects."]
    _1 = 1,
}
impl From<HSMCF_A> for bool {
    #[inline(always)]
    fn from(variant: HSMCF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSMCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMCF_A {
        match self.bits {
            false => HSMCF_A::_0,
            true => HSMCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSMCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSMCF_A::_1
    }
}
#[doc = "Field `HSMCF` writer - Hs-mode Master Code Detection Flag"]
pub type HSMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, HSMCF_A, O>;
impl<'a, const O: u8> HSMCF_W<'a, O> {
    #[doc = "Hs-mode Master Code does not detect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSMCF_A::_0)
    }
    #[doc = "Hs-mode Master Code detects."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSMCF_A::_1)
    }
}
#[doc = "Field `DVIDF` reader - Device-ID Address Detection Flag"]
pub type DVIDF_R = crate::BitReader<DVIDF_A>;
#[doc = "Device-ID Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVIDF_A {
    #[doc = "0: Device-ID command does not detect."]
    _0 = 0,
    #[doc = "1: Device-ID command detects. This bit set to 1 when the first frame received immediately after a START condition is detected matches a value of (device ID (1111 100) + 0\\[W\\])."]
    _1 = 1,
}
impl From<DVIDF_A> for bool {
    #[inline(always)]
    fn from(variant: DVIDF_A) -> Self {
        variant as u8 != 0
    }
}
impl DVIDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVIDF_A {
        match self.bits {
            false => DVIDF_A::_0,
            true => DVIDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVIDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVIDF_A::_1
    }
}
#[doc = "Field `DVIDF` writer - Device-ID Address Detection Flag"]
pub type DVIDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, DVIDF_A, O>;
impl<'a, const O: u8> DVIDF_W<'a, O> {
    #[doc = "Device-ID command does not detect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVIDF_A::_0)
    }
    #[doc = "Device-ID command detects. This bit set to 1 when the first frame received immediately after a START condition is detected matches a value of (device ID (1111 100) + 0\\[W\\])."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVIDF_A::_1)
    }
}
#[doc = "Field `HOAF` reader - Host Address Detection Flag"]
pub type HOAF_R = crate::BitReader<HOAF_A>;
#[doc = "Host Address Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOAF_A {
    #[doc = "0: Host address does not detect."]
    _0 = 0,
    #[doc = "1: Host address detects. This bit set to 1 when the received slave address matches the host address (0001 000)."]
    _1 = 1,
}
impl From<HOAF_A> for bool {
    #[inline(always)]
    fn from(variant: HOAF_A) -> Self {
        variant as u8 != 0
    }
}
impl HOAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOAF_A {
        match self.bits {
            false => HOAF_A::_0,
            true => HOAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOAF_A::_1
    }
}
#[doc = "Field `HOAF` writer - Host Address Detection Flag"]
pub type HOAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, HOAF_A, O>;
impl<'a, const O: u8> HOAF_W<'a, O> {
    #[doc = "Host address does not detect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOAF_A::_0)
    }
    #[doc = "Host address detects. This bit set to 1 when the received slave address matches the host address (0001 000)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOAF_A::_1)
    }
}
#[doc = "Field `SVAF0` reader - Slave Address Detection Flag 0"]
pub type SVAF0_R = crate::BitReader<SVAF0_A>;
#[doc = "Slave Address Detection Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAF0_A {
    #[doc = "0: Slave 0 does not detect"]
    _0 = 0,
    #[doc = "1: Slave 0 detect"]
    _1 = 1,
}
impl From<SVAF0_A> for bool {
    #[inline(always)]
    fn from(variant: SVAF0_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAF0_A {
        match self.bits {
            false => SVAF0_A::_0,
            true => SVAF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAF0_A::_1
    }
}
#[doc = "Field `SVAF0` writer - Slave Address Detection Flag 0"]
pub type SVAF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, SVAF0_A, O>;
impl<'a, const O: u8> SVAF0_W<'a, O> {
    #[doc = "Slave 0 does not detect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAF0_A::_0)
    }
    #[doc = "Slave 0 detect"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAF0_A::_1)
    }
}
#[doc = "Field `SVAF1` reader - Slave Address Detection Flag 1"]
pub type SVAF1_R = crate::BitReader<SVAF1_A>;
#[doc = "Slave Address Detection Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAF1_A {
    #[doc = "0: Slave 1 does not detect"]
    _0 = 0,
    #[doc = "1: Slave 1 detect"]
    _1 = 1,
}
impl From<SVAF1_A> for bool {
    #[inline(always)]
    fn from(variant: SVAF1_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAF1_A {
        match self.bits {
            false => SVAF1_A::_0,
            true => SVAF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAF1_A::_1
    }
}
#[doc = "Field `SVAF1` writer - Slave Address Detection Flag 1"]
pub type SVAF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, SVAF1_A, O>;
impl<'a, const O: u8> SVAF1_W<'a, O> {
    #[doc = "Slave 1 does not detect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAF1_A::_0)
    }
    #[doc = "Slave 1 detect"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAF1_A::_1)
    }
}
#[doc = "Field `SVAF2` reader - Slave Address Detection Flag 2"]
pub type SVAF2_R = crate::BitReader<SVAF2_A>;
#[doc = "Slave Address Detection Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAF2_A {
    #[doc = "0: Slave 2 does not detect"]
    _0 = 0,
    #[doc = "1: Slave 2 detect"]
    _1 = 1,
}
impl From<SVAF2_A> for bool {
    #[inline(always)]
    fn from(variant: SVAF2_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAF2_A {
        match self.bits {
            false => SVAF2_A::_0,
            true => SVAF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAF2_A::_1
    }
}
#[doc = "Field `SVAF2` writer - Slave Address Detection Flag 2"]
pub type SVAF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVST_SPEC, SVAF2_A, O>;
impl<'a, const O: u8> SVAF2_W<'a, O> {
    #[doc = "Slave 2 does not detect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAF2_A::_0)
    }
    #[doc = "Slave 2 detect"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAF2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - General Call Address Detection Flag"]
    #[inline(always)]
    pub fn gcaf(&self) -> GCAF_R {
        GCAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Hs-mode Master Code Detection Flag"]
    #[inline(always)]
    pub fn hsmcf(&self) -> HSMCF_R {
        HSMCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device-ID Address Detection Flag"]
    #[inline(always)]
    pub fn dvidf(&self) -> DVIDF_R {
        DVIDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Host Address Detection Flag"]
    #[inline(always)]
    pub fn hoaf(&self) -> HOAF_R {
        HOAF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Address Detection Flag 0"]
    #[inline(always)]
    pub fn svaf0(&self) -> SVAF0_R {
        SVAF0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Address Detection Flag 1"]
    #[inline(always)]
    pub fn svaf1(&self) -> SVAF1_R {
        SVAF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Address Detection Flag 2"]
    #[inline(always)]
    pub fn svaf2(&self) -> SVAF2_R {
        SVAF2_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn gcaf(&mut self) -> GCAF_W<0> {
        GCAF_W::new(self)
    }
    #[doc = "Bit 5 - Hs-mode Master Code Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsmcf(&mut self) -> HSMCF_W<5> {
        HSMCF_W::new(self)
    }
    #[doc = "Bit 6 - Device-ID Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dvidf(&mut self) -> DVIDF_W<6> {
        DVIDF_W::new(self)
    }
    #[doc = "Bit 15 - Host Address Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hoaf(&mut self) -> HOAF_W<15> {
        HOAF_W::new(self)
    }
    #[doc = "Bit 16 - Slave Address Detection Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn svaf0(&mut self) -> SVAF0_W<16> {
        SVAF0_W::new(self)
    }
    #[doc = "Bit 17 - Slave Address Detection Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn svaf1(&mut self) -> SVAF1_W<17> {
        SVAF1_W::new(self)
    }
    #[doc = "Bit 18 - Slave Address Detection Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn svaf2(&mut self) -> SVAF2_W<18> {
        SVAF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svst](index.html) module"]
pub struct SVST_SPEC;
impl crate::RegisterSpec for SVST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svst::R](R) reader structure"]
impl crate::Readable for SVST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svst::W](W) writer structure"]
impl crate::Writable for SVST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVST to value 0"]
impl crate::Resettable for SVST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
