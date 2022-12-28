#[doc = "Register `ECSR` reader"]
pub struct R(crate::R<ECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECSR` writer"]
pub struct W(crate::W<ECSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECSR_SPEC>;
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
impl From<crate::W<ECSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICD` reader - False Carrier Detect Flag"]
pub type ICD_R = crate::BitReader<ICD_A>;
#[doc = "False Carrier Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICD_A {
    #[doc = "0: PHY-LSI has not detected a false carrier on the line"]
    _0 = 0,
    #[doc = "1: PHY-LSI detected a false carrier on the line."]
    _1 = 1,
}
impl From<ICD_A> for bool {
    #[inline(always)]
    fn from(variant: ICD_A) -> Self {
        variant as u8 != 0
    }
}
impl ICD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICD_A {
        match self.bits {
            false => ICD_A::_0,
            true => ICD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICD_A::_1
    }
}
#[doc = "Field `ICD` writer - False Carrier Detect Flag"]
pub type ICD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSR_SPEC, ICD_A, O>;
impl<'a, const O: u8> ICD_W<'a, O> {
    #[doc = "PHY-LSI has not detected a false carrier on the line"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICD_A::_0)
    }
    #[doc = "PHY-LSI detected a false carrier on the line."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICD_A::_1)
    }
}
#[doc = "Field `MPD` reader - Magic Packet Detect Flag"]
pub type MPD_R = crate::BitReader<MPD_A>;
#[doc = "Magic Packet Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPD_A {
    #[doc = "0: Magic Packet not detected"]
    _0 = 0,
    #[doc = "1: Magic Packet detected."]
    _1 = 1,
}
impl From<MPD_A> for bool {
    #[inline(always)]
    fn from(variant: MPD_A) -> Self {
        variant as u8 != 0
    }
}
impl MPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPD_A {
        match self.bits {
            false => MPD_A::_0,
            true => MPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPD_A::_1
    }
}
#[doc = "Field `MPD` writer - Magic Packet Detect Flag"]
pub type MPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSR_SPEC, MPD_A, O>;
impl<'a, const O: u8> MPD_W<'a, O> {
    #[doc = "Magic Packet not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPD_A::_0)
    }
    #[doc = "Magic Packet detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPD_A::_1)
    }
}
#[doc = "Field `LCHNG` reader - Link Signal Change Flag"]
pub type LCHNG_R = crate::BitReader<LCHNG_A>;
#[doc = "Link Signal Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCHNG_A {
    #[doc = "0: Change in the ET0_LINKSTA signal not detected"]
    _0 = 0,
    #[doc = "1: Change in the ET0_LINKSTA signal detected (high to low, or low to high)."]
    _1 = 1,
}
impl From<LCHNG_A> for bool {
    #[inline(always)]
    fn from(variant: LCHNG_A) -> Self {
        variant as u8 != 0
    }
}
impl LCHNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCHNG_A {
        match self.bits {
            false => LCHNG_A::_0,
            true => LCHNG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCHNG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCHNG_A::_1
    }
}
#[doc = "Field `LCHNG` writer - Link Signal Change Flag"]
pub type LCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSR_SPEC, LCHNG_A, O>;
impl<'a, const O: u8> LCHNG_W<'a, O> {
    #[doc = "Change in the ET0_LINKSTA signal not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCHNG_A::_0)
    }
    #[doc = "Change in the ET0_LINKSTA signal detected (high to low, or low to high)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCHNG_A::_1)
    }
}
#[doc = "Field `PSRTO` reader - PAUSE Frame Retransmit Over Flag"]
pub type PSRTO_R = crate::BitReader<PSRTO_A>;
#[doc = "PAUSE Frame Retransmit Over Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSRTO_A {
    #[doc = "0: PAUSE frame retransmit count has not reached the upper limit"]
    _0 = 0,
    #[doc = "1: PAUSE frame retransmit count reached the upper limit."]
    _1 = 1,
}
impl From<PSRTO_A> for bool {
    #[inline(always)]
    fn from(variant: PSRTO_A) -> Self {
        variant as u8 != 0
    }
}
impl PSRTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSRTO_A {
        match self.bits {
            false => PSRTO_A::_0,
            true => PSRTO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSRTO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSRTO_A::_1
    }
}
#[doc = "Field `PSRTO` writer - PAUSE Frame Retransmit Over Flag"]
pub type PSRTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSR_SPEC, PSRTO_A, O>;
impl<'a, const O: u8> PSRTO_W<'a, O> {
    #[doc = "PAUSE frame retransmit count has not reached the upper limit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSRTO_A::_0)
    }
    #[doc = "PAUSE frame retransmit count reached the upper limit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSRTO_A::_1)
    }
}
#[doc = "Field `BFR` reader - Continuous Broadcast Frame Reception Flag"]
pub type BFR_R = crate::BitReader<BFR_A>;
#[doc = "Continuous Broadcast Frame Reception Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFR_A {
    #[doc = "0: Continuous reception of broadcast frames not detected"]
    _0 = 0,
    #[doc = "1: Continuous reception of broadcast frames detected."]
    _1 = 1,
}
impl From<BFR_A> for bool {
    #[inline(always)]
    fn from(variant: BFR_A) -> Self {
        variant as u8 != 0
    }
}
impl BFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFR_A {
        match self.bits {
            false => BFR_A::_0,
            true => BFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFR_A::_1
    }
}
#[doc = "Field `BFR` writer - Continuous Broadcast Frame Reception Flag"]
pub type BFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSR_SPEC, BFR_A, O>;
impl<'a, const O: u8> BFR_W<'a, O> {
    #[doc = "Continuous reception of broadcast frames not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFR_A::_0)
    }
    #[doc = "Continuous reception of broadcast frames detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - False Carrier Detect Flag"]
    #[inline(always)]
    pub fn icd(&self) -> ICD_R {
        ICD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Detect Flag"]
    #[inline(always)]
    pub fn mpd(&self) -> MPD_R {
        MPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Link Signal Change Flag"]
    #[inline(always)]
    pub fn lchng(&self) -> LCHNG_R {
        LCHNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PAUSE Frame Retransmit Over Flag"]
    #[inline(always)]
    pub fn psrto(&self) -> PSRTO_R {
        PSRTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Continuous Broadcast Frame Reception Flag"]
    #[inline(always)]
    pub fn bfr(&self) -> BFR_R {
        BFR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - False Carrier Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icd(&mut self) -> ICD_W<0> {
        ICD_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpd(&mut self) -> MPD_W<1> {
        MPD_W::new(self)
    }
    #[doc = "Bit 2 - Link Signal Change Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lchng(&mut self) -> LCHNG_W<2> {
        LCHNG_W::new(self)
    }
    #[doc = "Bit 4 - PAUSE Frame Retransmit Over Flag"]
    #[inline(always)]
    #[must_use]
    pub fn psrto(&mut self) -> PSRTO_W<4> {
        PSRTO_W::new(self)
    }
    #[doc = "Bit 5 - Continuous Broadcast Frame Reception Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bfr(&mut self) -> BFR_W<5> {
        BFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecsr](index.html) module"]
pub struct ECSR_SPEC;
impl crate::RegisterSpec for ECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecsr::R](R) reader structure"]
impl crate::Readable for ECSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecsr::W](W) writer structure"]
impl crate::Writable for ECSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECSR to value 0"]
impl crate::Resettable for ECSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
